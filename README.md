# cargo-sls-distribution

A Cargo subcommand which packages binary crates in a format compatible with the [SLS specification][SLS] for easy
distribution and execution. The package layout is designed to split immutable files from mutable state and
configuration.

The crate is packaged with a simple daemonizing script, a manifest describing the content of the package, and other
user-defined content:

```
[service-name]-[service-version]/
    deployment/
        manifest.yml                      # simple package manifest
    service/
        bin/
            [service-name]                # crate executable
            init.sh                       # daemonizing script
    var/                                  # application configuration and data
```

Packages are produced as a gzipped tarball named `[service-name]-[service-version].sls.tgz`.

[SLS]: https://github.com/palantir/sls-spec

## Usage

Install via Cargo and run as a subcommand:

```
$ cargo install --git https://github.com/sfackler/cargo-sls-distribution cargo-sls-distribution
    ...
$ cargo sls-distribution
```

The path of the created package will be printed to standard out.

## Configuration

Configuration is specified in the `package.metadata.sls-distribution` section of your Cargo.toml.

```toml
[package.metadata.sls-distribution]
# A Maven-style group name for the distribution.
# Required.
product-group = "com.foobar"

# A list of command line arguments to supply to the crate when running it.
# Defaults to an empty list.
args = ["var/conf/server.yml"]

# If set, the service version will be derived from `git describe` rather than the Cargo package version.
# Defaults to false.
git_version = true

# A map of extended manifest attributes.
# Defaults to an empty map.
manifest-extensions = { key = "value" }

# An array information about services that this depends on.
[[package.metadata.sls-distribution.product-dependencies]]
product-group = "com.foobar"
product-name = "my-service"
minimum-version = "1.1.0"
```

The contents of the `deployment`, `service`, and `var` directories will be added to the archive if present, though this
can be controlled as you would for `cargo package`, via the standard `package.include` and `package.exclude` fields as
well as `.gitignore` files:

```toml
[package]
exclude = ["var/data/*"]
```
