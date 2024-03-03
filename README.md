# clipper2

A polygon Clipping and Offsetting library for Rust.

The create is a Rust wrapper around the C++ version of
[Clipper2](https://github.com/AngusJohnson/Clipper2).

[![crate.io](https://img.shields.io/crates/v/clipper2.svg)](https://crates.io/crates/clipper2)
[![docs.rs](https://docs.rs/clipper2/badge.svg)](https://docs.rs/clipper2)

Compile with cargo feature `generate-bindings` to generate bindings at build
time.

This project was inspired from the
[clipper-sys](https://crates.io/crates/clipper-sys) crate that wraps the
previous version 1 of the Clipper library with some minor differences. The 
intent of this crate is similar, to build the Clipper(2) library
automatically, but ALSO directly expose SAFE types and functions to work with
directly in Rust where clipper-sys is more targeted towards use through the
[geo-clipper](https://crates.io/crates/geo-clipper) crate.

## Early days

This project is in a super early stage and has for now only opened up a small
part of what the C++ Clipper2 library has to offer. Expect breaking changes now
and then for some more time to come as we find more eregonomic ways of ecposing
the API.

Please also feel free to come with suggestions on how the API can be simplified
or send code contributions directly. See
[CONTRIBUTING.md](https://github.com/tirithen/clipper2/blob/main/CONTRIBUTING.md)
for more details.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/tirithen/clipper2/blob/main/LICENSE-APACHE.md)
or [MIT license](https://github.com/tirithen/clipper2/blob/main/LICENSE-MIT.md)
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
