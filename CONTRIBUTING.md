# Contributing

Code contributions that opens up more of the Clipper2 API for safe Rust
is more than welcome.

## Feature requests

If you find a feature missing, please open a github issue at
[](https://github.com/tirithen/clipper2/issues).

## Code contributions

The commit messages merged into this project should follow the
Conventional Commits specification
(for details see [](https://www.conventionalcommits.org/en/v1.0.0/)).

Each merge request/branch should contain one feature or fix and be squashed 
into one single commit following the conventional commits annotation. The
reason behind this is that this enables automatic changelog generation that
makes it clear which features, bug fixes and breaking changes each release
provides.

## Bindings

If you update clipper or bindgen, run `cargo build --features update-bindings`. This requires
libclang.
