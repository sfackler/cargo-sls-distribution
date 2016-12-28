extern crate cargo;
extern crate docopt;
extern crate flate2;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_yaml;
extern crate tar;
extern crate toml;
extern crate walkdir;

use cargo::core::Workspace;
use cargo::core::package::Package;
use cargo::ops::{self, CompileOptions, MessageFormat};
use cargo::util::important_paths::find_root_manifest_for_wd;
use cargo::util::errors::{ChainError, internal};
use cargo::{Config, CliResult, human};
use flate2::Compression;
use flate2::write::GzEncoder;
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::time::UNIX_EPOCH;
use std::path::Path;
use walkdir::WalkDir;

use serde_types::{CargoToml, CargoDistribution, Manifest};

mod serde_types;

const USAGE: &'static str = "
Package a binary crate into a distribution tarball

Usage: cargo distribution [options]
       cargo distribution --help

Options:
    -h, --help              Display this message
    -V, --version           Print version info and exit
    -j N, --jobs N          Number of parallel jobs, defaults to # of CPUs
    --bin NAME              Package this binary
    --release                    Build artifacts in release mode, with optimizations
    --features FEATURES     Space separated list of features to include
    --all-features          Build all available features
    --no-default-features   Do not include the `default` feature
    --target TARGET         Set the target triple
    --manifest-path PATH    Path to the manifest to use
    -v, --verbose ...       Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet             No output is printed
    --color WHEN            Coloring: auto, always, never
    --message-format FMT    Error format: human, json [default: human]
    --frozen                Require Cargo.lock and cache are up to date
    --locked                Require Cargo.lock is up to date
";

#[derive(RustcDecodable)]
struct Flags {
    flag_version: bool,
    flag_jobs: Option<u32>,
    flag_bin: Vec<String>,
    flag_release: bool,
    flag_features: Vec<String>,
    flag_all_features: bool,
    flag_no_default_features: bool,
    flag_target: Option<String>,
    flag_manifest_path: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_message_format: MessageFormat,
    flag_frozen: bool,
    flag_locked: bool,
}

fn main() {
    cargo::execute_main_without_stdin(real_main, false, USAGE);
}

fn real_main(options: Flags, config: &Config) -> CliResult<Option<()>> {
    if options.flag_version {
        println!("cargo-distribution {}", env!("CARGO_PKG_VERSION"));
        return Ok(None);
    }

    config.configure(options.flag_verbose,
                     options.flag_quiet,
                     &options.flag_color,
                     options.flag_frozen,
                     options.flag_locked)?;

    let root = find_root_manifest_for_wd(options.flag_manifest_path, config.cwd())?;

    let opts = CompileOptions {
        config: config,
        jobs: options.flag_jobs,
        target: options.flag_target.as_ref().map(|t| &t[..]),
        features: &options.flag_features,
        all_features: options.flag_all_features,
        no_default_features: options.flag_no_default_features,
        spec: &[],
        mode: ops::CompileMode::Build,
        release: options.flag_release,
        filter: ops::CompileFilter::new(false, &options.flag_bin, &[], &[], &[]),
        message_format: options.flag_message_format,
        target_rustdoc_args: None,
        target_rustc_args: None,
    };

    let ws = Workspace::new(&root, config)?;
    let package = ws.current()?;

    let config = get_config(package)?;

    let compilation = ops::compile(&ws, &opts)?;

    if compilation.binaries.len() != 1 {
        return Err(human(format!("expected a single binary, but saw {}",
                                 compilation.binaries.len())).into());
    }

    build_dist(package, config, &compilation.binaries[0])?;

    Ok(None)
}

fn get_config(package: &Package) -> CliResult<CargoDistribution> {
    let mut config = String::new();
    File::open(package.manifest_path())
        .chain_error(|| human("error opening Cargo.toml"))?
        .read_to_string(&mut config)
        .chain_error(|| human("error reading Cargo.toml"))?;

    let mut parser = toml::Parser::new(&config);
    let table = parser.parse().ok_or_else(|| human("error parsing Cargo.toml"))?;

    CargoToml::deserialize(&mut toml::Decoder::new(toml::Value::Table(table)))
        .chain_error(|| human("error deserializing Cargo.toml"))
        .map(|t| t.package.metadata.distribution)
        .map_err(Into::into)
}

fn build_dist(package: &Package, config: CargoDistribution, binary_path: &Path) -> CliResult<()> {
    let name = package.name();
    let version = package.version();
    let identifier = format!("{}-{}", name, version);
    let package_dir = package.manifest_path().parent().unwrap();
    let base = Path::new(&identifier);
    let bin_dir = base.join("service/bin");

    let out = binary_path.parent().unwrap().join(format!("{}.sls.tgz", identifier));
    let out = File::create(&out)
        .chain_error(|| human(format!("error creating tarball {}", out.display())))?;
    let out = BufWriter::new(out);
    let out = GzEncoder::new(out, Compression::Default);
    let mut out = tar::Builder::new(out);

    let manifest = Manifest {
        manifest_version: "1.0".to_owned(),
        product_type: "service.v1".to_owned(),
        product_group: config.group,
        product_name: package.name().to_owned(),
        product_version: package.version().to_string(),
        extensions: config.manifest_extensions,
    };
    let manifest = serde_yaml::to_string(&manifest).unwrap();
    add_string(&mut out, &manifest, &base.join("deployment/manifest.yml"))?;

    add_file(&mut out, binary_path, &bin_dir.join(binary_path.file_name().unwrap()))?;

    add_dir(&mut out, &package_dir.join("var"), &base.join("var"))?;
    add_dir(&mut out, &package_dir.join("deployment"), &base.join("deployment"))?;
    add_dir(&mut out, &package_dir.join("service"), &base.join("service"))?;

    out.into_inner()
        .and_then(|w| w.finish())
        .and_then(|mut w| w.flush())
        .chain_error(|| human("error writing tarball"))?;

    Ok(())
}

fn add_file<W>(out: &mut tar::Builder<W>, file_path: &Path, target_path: &Path) -> CliResult<()>
    where W: Write
{
    let mut file = File::open(file_path)
        .chain_error(|| human(format!("error opening file {}", file_path.display())))?;
    out.append_file(target_path, &mut file).chain_error(|| human("error writing tarball"))?;
    Ok(())
}

fn add_string<W>(out: &mut tar::Builder<W>, contents: &str, target_path: &Path) -> CliResult<()>
    where W: Write
{
    let mut header = tar::Header::new_gnu();
    header.set_path(target_path).chain_error(|| human("error writing tarball"))?;
    header.set_size(contents.len() as u64);
    header.set_entry_type(tar::EntryType::file());
    header.set_mtime(UNIX_EPOCH.elapsed().unwrap().as_secs());
    header.set_mode(0o644);
    header.set_cksum();

    out.append(&header, &mut contents.as_bytes())
        .chain_error(|| human("error writing tarball"))?;
    Ok(())
}

fn add_dir<W>(out: &mut tar::Builder<W>, source_path: &Path, target_path: &Path) -> CliResult<()>
    where W: Write
{
    if !source_path.is_dir() {
        return Ok(());
    }

    for entry in WalkDir::new(source_path) {
        let entry = entry.map_err(|e| internal(e))?;
        if entry.file_type().is_file() {
            let path = entry.path().strip_prefix(source_path).unwrap();
            add_file(out, entry.path(), &target_path.join(path))?;
        }
    }

    Ok(())
}
