extern crate cargo;
extern crate docopt;
extern crate flate2;
extern crate rustc_serialize;
extern crate tar;

use cargo::core::Workspace;
use cargo::core::package::Package;
use cargo::ops::{self, CompileOptions, MessageFormat};
use cargo::util::important_paths::find_root_manifest_for_wd;
use cargo::util::errors::ChainError;
use cargo::{Config, CliResult, human};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;

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
    let compilation = ops::compile(&ws, &opts)?;

    if compilation.binaries.len() != 1 {
        return Err(human(format!("expected a single binary, but saw {}",
                                 compilation.binaries.len())).into());
    }

    build_dist(package, &compilation.binaries[0])?;

    Ok(None)
}

fn build_dist(package: &Package, bin_path: &Path) -> CliResult<()> {
    let name = package.name();
    let version = package.version();
    let identifier = format!("{}-{}", name, version);
    let base = Path::new(&identifier);
    let bin_dir = base.join("service/bin");

    let out = bin_path.parent().unwrap().join(format!("{}.sls.tgz", identifier));
    let out = File::create(&out)
        .chain_error(|| human(format!("error creating tarball {}", out.display())))?;
    let out = BufWriter::new(out);
    let out = GzEncoder::new(out, Compression::Default);
    let mut out = tar::Builder::new(out);

    let mut bin = File::open(bin_path).chain_error(|| human("error opening binary"))?;
    out.append_file(bin_dir.join(bin_path.file_name().unwrap()), &mut bin)
        .chain_error(|| human("error writing to tarball"))?;

    out.into_inner()
        .and_then(|w| w.finish())
        .and_then(|mut w| w.flush())
        .chain_error(|| human("error writing to tarball"))?;

    Ok(())
}
