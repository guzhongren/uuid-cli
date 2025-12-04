# uuid-cli

Small CLI to generate UUID values.

## Quick Start

- Build from source:

```sh
cargo build --release
```

- Run tests:

```sh
cargo test
```

## Releases

Prebuilt release artifacts are published on GitHub Releases. Artifact naming follows this pattern:

- `uuid-cli-<tag>-<target>.tar.gz` (example: `uuid-cli-v0.1.7-aarch64-apple-darwin.tar.gz`)
- Debian packages (when built for a tag) are named: `uuid-cli_<version>_<arch>.deb` (example: `uuid-cli_0.1.7_arm64.deb`).

Each tarball contains the `uuid-cli` binary, `README.md`, `LICENSE`, and optionally completions and a manpage when available.

## Usage

```sh
# generate one UUID
cargo run --release -- --count 1

# generate 5, uppercase, no hyphens
cargo run --release -- -n 5 --upper --no-hyphen

# run installed binary
# uuid-cli --count 10
```

## Cross-compilation / CI

The repository uses GitHub Actions to build for multiple targets. CI packages artifacts using the `PROJECT_NAME` and git tag (or short sha), producing names like `uuid-cli-<tag|sha>-<target>.tar.gz` which are uploaded and (on tagged releases) published to GitHub Releases.

If you need a local cross-build setup, consider using `cross` or setting up the appropriate toolchains for your target.

## Contributing

- Build and run tests locally with `cargo build --release` and `cargo test`.
- Open issues or PRs for bugs and improvements. Follow the repository's contribution guidelines when available.

## License

See the `LICENSE` file in the repository.

---

If you'd like, I can add a short example of installing a release tarball or a quick troubleshooting note for CI builds.
