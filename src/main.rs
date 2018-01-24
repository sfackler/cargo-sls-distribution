extern crate docopt;
extern crate env_logger;
extern crate flate2;
extern crate git2;
extern crate serde;
extern crate serde_json;
extern crate shell_escape;
extern crate tar;
extern crate toml;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use flate2::Compression;
use flate2::write::GzEncoder;
use git2::{Repository, DescribeOptions, DescribeFormatOptions};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::ffi::{OsString, OsStr};
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::UNIX_EPOCH;

use errors::*;

const USAGE: &'static str = "
Package a binary crate into a distribution tarball

Usage: cargo sls-distribution [options]

Options:
    -h, --help              Display this message
    -V, --version           Print version info and exit
    -j N, --jobs N          Number of parallel jobs, defaults to # of CPUs
    --bin NAME              Package this binary
    --release               Build artifacts in release mode, with optimizations
    --features FEATURES     Space separated list of features to include
    --all-features          Build all available features
    --no-default-features   Do not include the `default` feature
    --target TARGET         Set the target triple
    --manifest-path PATH    Path to the manifest to use
    -v, --verbose ...       Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet             No output is printed
    --color WHEN            Coloring: auto, always, never
    --frozen                Require Cargo.lock and cache are up to date
    --locked                Require Cargo.lock is up to date
";

const INIT_SH: &'static str = include_str!("init.sh");
const CHECK_SH: &'static str = include_str!("check.sh");

mod errors {
    error_chain!{}
}

#[derive(Deserialize)]
struct Flags {
    flag_jobs: Option<u32>,
    flag_bin: Vec<String>,
    flag_release: bool,
    flag_features: Vec<String>,
    flag_all_features: bool,
    flag_no_default_features: bool,
    flag_target: Option<String>,
    flag_manifest_path: Option<String>,
    flag_verbose: u32,
    flag_quiet: bool,
    flag_color: Option<String>,
    flag_frozen: bool,
    flag_locked: bool,
}

#[derive(Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
enum BuildMessage {
    CompilerArtifact {
        filenames: Vec<String>,
        target: TargetMetadata,
    },
}

#[derive(Deserialize)]
struct TargetMetadata {
    kind: Vec<String>,
    name: String,
}

#[derive(Deserialize)]
struct ProjectInfo {
    root: String,
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoPackage,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
    version: String,
    metadata: CargoMetadata,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct CargoMetadata {
    sls_distribution: SlsDistribution,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
struct SlsDistribution {
    product_group: String,
    #[serde(default)]
    args: Vec<String>,
    check_args: Option<Vec<String>>,
    #[serde(default)]
    git_version: bool,
    #[serde(default)]
    manifest_extensions: HashMap<String, Value>,
    #[serde(default)]
    product_dependencies: Vec<ProductDependency>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct ProductDependency {
    product_group: String,
    product_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    minimum_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    maximum_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recommended_version: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct Manifest {
    manifest_version: String,
    product_type: String,
    product_group: String,
    product_name: String,
    product_version: String,
    extensions: HashMap<String, Value>,
}

#[derive(Debug)]
struct Artifact {
    name: String,
    path: PathBuf,
}

quick_main!(real_main);

fn real_main() -> Result<()> {
    env_logger::init();

    let flags: Flags = Docopt::new(USAGE)
        .map(|d| d.help(true))
        .map(|d| {
            d.version(Some(format!(
                "cargo-sls-distribution {}",
                env!("CARGO_PKG_VERSION")
            )))
        })
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));

    let artifacts = build(&flags, &cargo)?;
    if artifacts.len() != 1 {
        bail!(
            "expected a single binary artifact, but got {}",
            artifacts.len()
        );
    }

    let manifest_path = get_manifest_path(&flags, &cargo)?;
    let project_root = manifest_path.parent().unwrap();

    let files = get_package_files(&flags, &project_root, &cargo)?;

    let config = get_config(&manifest_path)?;

    let version = get_version(&project_root, &config)?;

    let path = build_dist(&artifacts[0], &files, config, &project_root, &version)?;

    if !flags.flag_quiet {
        println!("{}", path.display());
    }

    Ok(())
}

// we call `cargo build` twice - once with normal human message to actually run
// the build and again with JSON messages to find where the binary is.
fn build(flags: &Flags, cargo: &OsStr) -> Result<Vec<Artifact>> {
    let mut command = Command::new(cargo);
    command.arg("build");
    if let Some(jobs) = flags.flag_jobs {
        command.arg("-j").arg(jobs.to_string());
    }
    for bin in &flags.flag_bin {
        command.arg("--bin").arg(bin);
    }
    if flags.flag_release {
        command.arg("--release");
    }
    if !flags.flag_features.is_empty() {
        command.arg("--features").arg(flags.flag_features.join(" "));
    }
    if flags.flag_all_features {
        command.arg("--all-features");
    }
    if flags.flag_no_default_features {
        command.arg("--no-default-features");
    }
    if let Some(ref target) = flags.flag_target {
        command.arg("--target").arg(target);
    }
    if let Some(ref manifest_path) = flags.flag_manifest_path {
        command.arg("--manifest-path").arg(manifest_path);
    }
    if flags.flag_quiet {
        command.arg("-q");
    }
    for _ in 0..flags.flag_verbose {
        command.arg("-v");
    }
    if let Some(ref color) = flags.flag_color {
        command.arg("--color").arg(color);
    }
    if flags.flag_frozen {
        command.arg("--frozen");
    }
    if flags.flag_locked {
        command.arg("--locked");
    }

    let status = command.status().chain_err(|| "error running cargo")?;
    if !status.success() {
        bail!("cargo build returned returned {}", status);
    }

    let mut command = Command::new(cargo);
    command
        .arg("build")
        .arg("--message-format")
        .arg("json")
        .arg("-q")
        .stdout(Stdio::piped());
    for bin in &flags.flag_bin {
        command.arg("--bin").arg(bin);
    }
    if flags.flag_release {
        command.arg("--release");
    }
    if !flags.flag_features.is_empty() {
        command.arg("--features").arg(flags.flag_features.join(" "));
    }
    if flags.flag_all_features {
        command.arg("--all-features");
    }
    if flags.flag_no_default_features {
        command.arg("--no-default-features");
    }
    if let Some(ref target) = flags.flag_target {
        command.arg("--target").arg(target);
    }
    if let Some(ref manifest_path) = flags.flag_manifest_path {
        command.arg("--manifest-path").arg(manifest_path);
    }

    let output = command
        .spawn()
        .and_then(|c| c.wait_with_output())
        .chain_err(|| "error running cargo")?;
    if !output.status.success() {
        bail!("cargo build returned {}", output.status);
    }

    let mut artifacts = vec![];
    for line in output.stdout.split(|c| *c == b'\n') {
        match serde_json::from_slice(line) {
            Ok(BuildMessage::CompilerArtifact {
                   ref target,
                   ref mut filenames,
               }) if target.kind == ["bin"] => {
                let artifact = Artifact {
                    name: target.name.clone(),
                    path: filenames.pop().unwrap().into(),
                };
                artifacts.push(artifact);
            }
            _ => debug!("skipping line `{}`", String::from_utf8_lossy(line)),
        }
    }
    Ok(artifacts)
}

fn get_package_files(flags: &Flags, project_root: &Path, cargo: &OsStr) -> Result<Vec<PathBuf>> {
    let mut command = Command::new(cargo);
    command
        .arg("package")
        .arg("-l")
        .arg("--no-metadata")
        .stdout(Stdio::piped());
    if let Some(ref manifest_path) = flags.flag_manifest_path {
        command.arg("--manifest-path").arg(manifest_path);
    }
    for _ in 0..flags.flag_verbose {
        command.arg("-v");
    }
    if flags.flag_quiet {
        command.arg("-q");
    }
    if let Some(ref color) = flags.flag_color {
        command.arg("--color").arg(color);
    }
    if flags.flag_frozen {
        command.arg("--frozen");
    }
    if flags.flag_locked {
        command.arg("--locked");
    }

    let output = command
        .spawn()
        .and_then(|c| c.wait_with_output())
        .chain_err(|| "error running cargo")?;
    if !output.status.success() {
        bail!("cargo package returned {}", output.status);
    }
    let stdout = String::from_utf8(output.stdout).chain_err(
        || "error parsing cargo package output",
    )?;

    let files = stdout.lines().map(|l| project_root.join(l)).collect();
    Ok(files)
}

fn get_manifest_path(flags: &Flags, cargo: &OsStr) -> Result<PathBuf> {
    let mut command = Command::new(cargo);
    command.arg("locate-project").stdout(Stdio::piped());
    if let Some(ref manifest_path) = flags.flag_manifest_path {
        command.arg("--manifest-path").arg(manifest_path);
    }

    let output = command
        .spawn()
        .and_then(|c| c.wait_with_output())
        .chain_err(|| "error running cargo")?;
    if !output.status.success() {
        bail!("cargo locate-project returned {}", output.status);
    }

    let info: ProjectInfo = serde_json::from_slice(&output.stdout).chain_err(
        || "error parsing cargo locate-project output",
    )?;
    Ok(info.root.into())
}

fn get_config(manifest_path: &Path) -> Result<CargoToml> {
    let mut config = String::new();
    File::open(manifest_path)
        .and_then(|mut f| f.read_to_string(&mut config))
        .chain_err(|| "error reading Cargo.toml")?;

    toml::from_str(&config).chain_err(|| "error parsing Cargo.toml")
}

fn get_version(project_root: &Path, config: &CargoToml) -> Result<String> {
    if config.package.metadata.sls_distribution.git_version {
        let repo = Repository::discover(project_root).chain_err(
            || "error discovering git repository",
        )?;
        let description = repo.describe(DescribeOptions::new().describe_tags())
            .chain_err(|| "error describing git repository")?;
        let version = description
            .format(Some(DescribeFormatOptions::new().dirty_suffix("-dirty")))
            .chain_err(|| "error formatting git description")?;
        Ok(version)
    } else {
        Ok(config.package.version.clone())
    }
}

fn build_dist(
    artifact: &Artifact,
    sources: &[PathBuf],
    config: CargoToml,
    package_dir: &Path,
    version: &str,
) -> Result<PathBuf> {
    let name = config.package.name;
    let identifier = format!("{}-{}", name, version);
    let base = Path::new(&identifier);

    let out_path = artifact.path.parent().unwrap().join(format!(
        "{}.sls.tgz",
        identifier
    ));
    let out = File::create(&out_path).chain_err(|| {
        format!("error creating tarball {}", out_path.display())
    })?;
    let out = BufWriter::new(out);
    let out = GzEncoder::new(out, Compression::default());
    let mut out = tar::Builder::new(out);

    let sls_distribution = config.package.metadata.sls_distribution;
    let mut extensions = sls_distribution.manifest_extensions;
    if !sls_distribution.product_dependencies.is_empty() {
        extensions.insert(
            "product-dependencies".to_string(),
            serde_json::to_value(sls_distribution.product_dependencies).unwrap(),
        );
    }

    let manifest = Manifest {
        manifest_version: "1.0".to_owned(),
        product_type: "service.v1".to_owned(),
        product_group: sls_distribution.product_group,
        product_name: name.clone(),
        product_version: version.to_string(),
        extensions: extensions,
    };
    let manifest = serde_json::to_string_pretty(&manifest).unwrap();
    add_string(
        &mut out,
        &manifest,
        &base.join("deployment/manifest.yml"),
        0o644,
    )?;

    let binary_path = Path::new("service/bin").join(artifact.path.file_name().unwrap());
    let start_args = sls_distribution
        .args
        .into_iter()
        .map(|s| shell_escape::escape(s.into()))
        .collect::<Vec<_>>()
        .join(" ");
    let check_args = match sls_distribution.check_args {
        Some(check_args) => {
            add_string(
                &mut out,
                CHECK_SH,
                &base.join("service/monitoring/bin/check.sh"),
                0o755,
            )?;
            check_args
                .into_iter()
                .map(|s| shell_escape::escape(s.into()))
                .collect::<Vec<_>>()
                .join(" ")
        }
        None => "".to_string(),
    };
    let init_sh = INIT_SH
        .replace("@bin@", &binary_path.display().to_string())
        .replace("@start_args@", &start_args)
        .replace("@check_args@", &check_args)
        .replace("@service@", &name);
    add_string(&mut out, &init_sh, &base.join("service/bin/init.sh"), 0o755)?;

    add_file(&mut out, &artifact.path, &base.join(&binary_path))?;

    add_dir(
        &mut out,
        sources,
        &package_dir.join("var"),
        &base.join("var"),
    )?;
    add_dir(
        &mut out,
        sources,
        &package_dir.join("deployment"),
        &base.join("deployment"),
    )?;
    add_dir(
        &mut out,
        sources,
        &package_dir.join("service"),
        &base.join("service"),
    )?;

    out.into_inner()
        .and_then(|w| w.finish())
        .and_then(|mut w| w.flush())
        .chain_err(|| "error writing tarball")?;

    Ok(out_path)
}

fn add_file<W>(out: &mut tar::Builder<W>, file_path: &Path, target_path: &Path) -> Result<()>
where
    W: Write,
{
    let mut file = File::open(file_path).chain_err(|| {
        format!("error opening file {}", file_path.display())
    })?;
    out.append_file(target_path, &mut file).chain_err(
        || "error writing tarball",
    )?;
    Ok(())
}

fn add_string<W>(
    out: &mut tar::Builder<W>,
    contents: &str,
    target_path: &Path,
    mode: u32,
) -> Result<()>
where
    W: Write,
{
    let mut header = tar::Header::new_gnu();
    header.set_path(target_path).chain_err(
        || "error writing tarball",
    )?;
    header.set_size(contents.len() as u64);
    header.set_entry_type(tar::EntryType::file());
    header.set_mtime(UNIX_EPOCH.elapsed().unwrap().as_secs());
    header.set_mode(mode);
    header.set_cksum();

    out.append(&header, &mut contents.as_bytes()).chain_err(
        || "error writing tarball",
    )?;
    Ok(())
}

fn add_dir<W>(
    out: &mut tar::Builder<W>,
    sources: &[PathBuf],
    source_path: &Path,
    target_path: &Path,
) -> Result<()>
where
    W: Write,
{
    for source in sources {
        if let Ok(prefix) = source.strip_prefix(source_path) {
            add_file(out, source, &target_path.join(prefix))?;
        }
    }

    Ok(())
}
