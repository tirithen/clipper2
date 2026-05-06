# Contributing to clipper2

Thanks for your interest. Issues, suggestions, and pull requests are
all welcome.

## Scope

This crate is the safe, idiomatic Rust API on top of Clipper2. The
raw FFI bindings live in
[`clipper2c-sys`](https://crates.io/crates/clipper2c-sys); contributions
that expose new C ABI items, fix unsafe-FFI bugs, or update the
vendored C++ source belong there. Contributions to this crate are
typically:

- exposing more of the Clipper2 surface as safe Rust,
- ergonomic improvements to the `Path` / `Paths` / `Clipper` APIs,
- additional path / polygon utilities and transforms,
- documentation and examples.

## Feature requests

If you find a feature missing, please open a GitHub issue at
<https://github.com/tirithen/clipper2/issues>.

## Local checks before opening a PR

```sh
cargo fmt
cargo test --all-features
scripts/wasm-check.sh    # downloads WASI SDK on first run, into .tmp/
```

CI runs the same `cargo test` on Linux, macOS, and Windows, plus the
`wasm32-unknown-unknown` build.

## Commit messages

This repo uses [Conventional Commits](https://www.conventionalcommits.org/).
The release tooling derives the next version and the CHANGELOG entry
directly from commit messages, so the type prefix matters:

- `feat:` — adds a new item to the public surface (patch bump on 0.x)
- `fix:` — bug fix (patch bump)
- `perf:` — performance improvement
- `docs:`, `test:`, `refactor:`, `chore:`, `ci:`, `build:`, `style:`
  — kept out of the changelog
- `BREAKING CHANGE:` footer — bumps the minor on 0.x (e.g. 0.5.4 →
  0.6.0). Use this for MSRV bumps, removed APIs, or signature
  changes.

Each pull request / branch should contain one feature or fix and be
squashed into a single commit following the Conventional Commits
annotation. The reason behind this is that it enables automatic
changelog generation that makes it clear which features, bug fixes,
and breaking changes each release provides.

Do not edit `Cargo.toml`'s `version` field or `CHANGELOG.md` by hand.
Those are produced by `scripts/release.sh` and the cargo-release /
git-cliff configuration in `release.toml` and `cliff.toml`.

## Licensing

By submitting a contribution you agree that it is dual-licensed
under [Apache-2.0](LICENSE-APACHE.md) and [MIT](LICENSE-MIT.md), as
stated at the bottom of the README.
