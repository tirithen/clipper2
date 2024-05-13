# clipper2

[![crate.io](https://img.shields.io/crates/v/clipper2.svg)](https://crates.io/crates/clipper2)
[![docs.rs](https://docs.rs/clipper2/badge.svg)](https://docs.rs/clipper2)

A path/polygon clipping and offsetting library for Rust.

The focus of this crate is to provide an easy to use API, staying as close to
the core `Vec` and `fn` types as possible.

The create uses the [clipper2c-sys](https://crates.io/crates/clipper2c-sys)
crate that in turn is a Rust wrapper around the C++ version of
[Clipper2](https://github.com/AngusJohnson/Clipper2).

## Example

The below example uses macroquad to visualize the result of the operations and
some helpers from the `examples/` directory. See the examples directory for more
examples.

```rust
use clipper2::*;
use helpers::{circle_path, draw_paths};
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Difference and inflate")]
async fn main() {
    let circle = circle_path((5.0, 5.0), 3.0, 32);
    let rectangle = vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)];
    let circle2 = circle_path((7.0, 7.0), 1.0, 32);

    let result = difference(circle, rectangle, FillRule::default())
        .expect("Failed to run boolean operation");

    let result = difference(result.clone(), circle2, FillRule::default())
        .expect("Failed to run boolean operation");

    let result2 = inflate(result.clone(), 1.0, JoinType::Round, EndType::Polygon, 0.0);

    loop {
        clear_background(BLACK);
        draw_paths(&result, SKYBLUE);
        draw_paths(&result2, GREEN);
        next_frame().await
    }
}
```

This is how the resulting shapes looks:

![Image displaying the result of the difference and inflate example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/difference-and-inflate.png)

## API uses f64, but i64 under the hood

The Clipper2 library is [using `i64` values](https://www.angusj.com/clipper2/Docs/Robustness.htm)
to guarantee the robustness of all calculations. The C++ library exposes both
`int64_t`/`i64` and `double`/`f64` versions of several types. This crate
therefore internally uses the `int64_t`/`i64` types only, but for now only
exposes an `f64` API.

The types `Point`, `Path`, and `Paths` therefore offers a `PointScaler` trait
and generic parameter that allows the user to choose the scaling is used when
it interally converts from `f64` to `i64`. By default it uses the `Centi` struct
that will scale the values by `100`.

## Early days

This project is in a super early stage and has for now only opened up a small
part of what the C++ Clipper2 library has to offer. Expect breaking changes now
and then for some more time to come as we find and explore more eregonomic ways
of exposing the API in a Rust ideomatic way.

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
