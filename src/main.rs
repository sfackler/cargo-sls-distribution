extern crate cargo;
extern crate docopt;
extern crate env_logger;
extern crate flate2;
extern crate git2;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_yaml;
extern crate shell_escape;
extern crate tar;
extern crate toml;

use cargo::core::package::Package;
use cargo::core::{Source, Workspace};
use cargo::core::shell::{Verbosity, ColorConfig};
use cargo::ops::{self, CompileOptions, MessageFormat, Packages};
use cargo::sources::PathSource;
use cargo::util::errors::ChainError;
use cargo::util::important_paths::find_root_manifest_for_wd;
use cargo::{Config, CargoResult, CliResult, human};
use flate2::Compression;
use flate2::write::GzEncoder;
use git2::{Repository, DescribeOptions, DescribeFormatOptions};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use serde_types::{CargoToml, CargoDistribution, Manifest};

mod serde_types;

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
    --message-format FMT    Error format: human, json [default: human]
    --frozen                Require Cargo.lock and cache are up to date
    --locked                Require Cargo.lock is up to date
";

const INIT_SH: &'static str = include_str!("init.sh");

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
    env_logger::init().unwrap();

    let config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => {
            let mut shell = cargo::shell(Verbosity::Verbose, ColorConfig::Auto);
            cargo::handle_cli_error(e.into(), &mut shell)
        }
    };

    let result = (|| {
        let args: Vec<_> = try!(env::args_os().map(|s| {
            s.into_string().map_err(|s| {
                human(format!("invalid unicode in argument: {:?}", s))
            })
        }).collect());
        cargo::call_main_without_stdin(real_main, &config, USAGE, &args, false)
    })();

    match result {
        Err(e) => cargo::handle_cli_error(e, &mut *config.shell()),
        Ok(()) => {},
    }
}

fn real_main(options: Flags, config: &Config) -> CliResult {
    if options.flag_version {
        println!("cargo-distribution {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
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
        spec: Packages::Packages(&[]),
        mode: ops::CompileMode::Build,
        release: options.flag_release,
        filter: ops::CompileFilter::new(false, &options.flag_bin, &[], &[], &[]),
        message_format: options.flag_message_format,
        target_rustdoc_args: None,
        target_rustc_args: None,
    };

    let ws = Workspace::new(&root, config)?;
    let package = ws.current()?;

    let mut sources = PathSource::new(package.root(), package.package_id().source_id(), config);
    sources.update()?;
    let sources = sources.list_files(package)?;

    let config = get_config(package)?;

    let version = get_version(package, &config)?;

    let compilation = ops::compile(&ws, &opts)?;

    if compilation.binaries.len() != 1 {
        return Err(human(format!("expected a single binary, but saw {}",
                                 compilation.binaries.len())).into());
    }

    let path = build_dist(package, &sources, config, version, &compilation.binaries[0])?;

    match options.flag_quiet {
        Some(true) => {}
        _ => println!("{}", path.display()),
    }

    Ok(())
}

fn get_config(package: &Package) -> CargoResult<CargoDistribution> {
    let mut config = String::new();
    File::open(package.manifest_path())
        .chain_error(|| human("error opening Cargo.toml"))?
        .read_to_string(&mut config)
        .chain_error(|| human("error reading Cargo.toml"))?;

    let mut parser = toml::Parser::new(&config);
    let table = parser.parse().ok_or_else(|| human("error parsing Cargo.toml"))?;

    CargoToml::deserialize(&mut toml::Decoder::new(toml::Value::Table(table)))
        .chain_error(|| human("error deserializing Cargo.toml"))
        .map(|t| t.package.metadata.sls_distribution)
        .map_err(Into::into)
}

fn get_version(package: &Package, config: &CargoDistribution) -> CargoResult<String> {
    if config.git_version {
        let repo = Repository::discover(package.root())
            .chain_error(|| human("error discovering git repository"))?;
        let description = repo.describe(DescribeOptions::new().describe_tags())
            .chain_error(|| human("error describing git repository"))?;
        let version = description.format(Some(DescribeFormatOptions::new().dirty_suffix("-dirty")))
            .chain_error(|| human("error formatting git version"))?;
        Ok(version)
    } else {
        Ok(package.version().to_string())
    }
}

fn build_dist(package: &Package,
              sources: &[PathBuf],
              config: CargoDistribution,
              version: String,
              binary_source: &Path)
              -> CargoResult<PathBuf> {
    let name = package.name();
    let identifier = format!("{}-{}", name, version);
    let package_dir = package.root();
    let base = Path::new(&identifier);

    let out_path = binary_source.parent().unwrap().join(format!("{}.sls.tgz", identifier));
    let out = File::create(&out_path)
        .chain_error(|| human(format!("error creating tarball {}", out_path.display())))?;
    let out = BufWriter::new(out);
    let out = GzEncoder::new(out, Compression::Default);
    let mut out = tar::Builder::new(out);

    let manifest = Manifest {
        manifest_version: "1.0".to_owned(),
        product_type: "service.v1".to_owned(),
        product_group: config.group,
        product_name: package.name().to_owned(),
        product_version: version,
        extensions: config.manifest_extensions,
    };
    let manifest = serde_yaml::to_string(&manifest).unwrap();
    add_string(&mut out, &manifest, &base.join("deployment/manifest.yml"), 0o644)?;

    let binary_path = Path::new("service/bin").join(binary_source.file_name().unwrap());
    let args = config.args
        .into_iter()
        .map(|s| shell_escape::escape(s.into()))
        .collect::<Vec<_>>()
        .join(" ");
    let init_sh = INIT_SH
        .replace("@bin@", &binary_path.display().to_string())
        .replace("@args@", &args)
        .replace("@service@", package.name());
    add_string(&mut out, &init_sh, &base.join("service/bin/init.sh"), 0o755)?;

    add_file(&mut out, binary_source, &base.join(&binary_path))?;

    add_dir(&mut out, sources, &package_dir.join("var"), &base.join("var"))?;
    add_dir(&mut out, sources, &package_dir.join("deployment"), &base.join("deployment"))?;
    add_dir(&mut out, sources, &package_dir.join("service"), &base.join("service"))?;

    out.into_inner()
        .and_then(|w| w.finish())
        .and_then(|mut w| w.flush())
        .chain_error(|| human("error writing tarball"))?;

    Ok(out_path)
}

fn add_file<W>(out: &mut tar::Builder<W>, file_path: &Path, target_path: &Path) -> CargoResult<()>
    where W: Write
{
    let mut file = File::open(file_path)
        .chain_error(|| human(format!("error opening file {}", file_path.display())))?;
    out.append_file(target_path, &mut file).chain_error(|| human("error writing tarball"))?;
    Ok(())
}

fn add_string<W>(out: &mut tar::Builder<W>,
                 contents: &str,
                 target_path: &Path,
                 mode: u32) -> CargoResult<()>
    where W: Write
{
    let mut header = tar::Header::new_gnu();
    header.set_path(target_path).chain_error(|| human("error writing tarball"))?;
    header.set_size(contents.len() as u64);
    header.set_entry_type(tar::EntryType::file());
    header.set_mtime(UNIX_EPOCH.elapsed().unwrap().as_secs());
    header.set_mode(mode);
    header.set_cksum();

    out.append(&header, &mut contents.as_bytes())
        .chain_error(|| human("error writing tarball"))?;
    Ok(())
}

fn add_dir<W>(out: &mut tar::Builder<W>,
              sources: &[PathBuf],
              source_path: &Path,
              target_path: &Path)
              -> CargoResult<()>
    where W: Write
{
    for source in sources {
        if let Ok(prefix) = source.strip_prefix(source_path) {
            add_file(out, source, &target_path.join(prefix))?;
        }
    }

    Ok(())
}
