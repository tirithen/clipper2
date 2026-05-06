# clipper2

[![crate.io](https://img.shields.io/crates/v/clipper2.svg)](https://crates.io/crates/clipper2)
[![docs.rs](https://docs.rs/clipper2/badge.svg)](https://docs.rs/clipper2)

A safe, idiomatic Rust API for 2D polygon clipping and offsetting,
built on top of [Clipper2](https://github.com/AngusJohnson/Clipper2),
Angus Johnson's C++ library.

The focus of this crate is to provide an easy to use API, staying as
close to the core `Vec` and `fn` types as possible. The unsafe FFI
layer lives in the separate
[`clipper2c-sys`](https://crates.io/crates/clipper2c-sys) crate; reach
for that one only if you need direct access to the raw C ABI.

## Example

The example below uses macroquad to visualize the result of the
operations and some helpers from the `examples/` directory. See the
examples directory for more.

```rust
use clipper2::*;
use helpers::{circle_path, draw_paths};
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Difference and inflate")]
async fn main() -> Result<(), ClipperError> {
    let circle = circle_path((5.0, 5.0), 3.0, 32);
    let circle2 = circle_path((7.0, 7.0), 1.0, 32);
    let rectangle = vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)];

    let result = circle
        .to_clipper_subject()
        .add_clip(circle2)
        .add_clip(rectangle)
        .difference(FillRule::default())?;

    let result2 = result
        .inflate(1.0, JoinType::Round, EndType::Polygon, 0.0)
        .simplify(0.01, false);

    loop {
        clear_background(BLACK);
        draw_paths(&result, SKYBLUE);
        draw_paths(&result2, GREEN);
        next_frame().await
    }
}
```

This is how the resulting shapes look:

![Image displaying the result of the difference and inflate example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/difference-and-inflate.png)

## Minkowski sum and difference

The Minkowski operations grow a polygon by an arbitrary polygonal
kernel rather than the radius-only kernel of `inflate`, which makes
them the right primitive for square cutters, drag-knives, asymmetric
tool footprints, and configuration-space obstacles in robot motion
planning. The kernel may be concave; concave kernels can carve holes
into the swept region.

`minkowski_sum` translates the kernel by `+p` at every vertex of the
input path; `minkowski_diff` translates by `-p`. For a pattern that
is symmetric about the origin (a centred disc or square) the two
agree, but for an asymmetric pattern the difference is the sum
reflected through the origin of the pattern. The screenshots below
use the same concave arrowhead kernel against the same L-shaped
outline:

```rust
use clipper2::*;

// A concave arrowhead pointing right (asymmetric, non-convex).
let pattern: Path = vec![
    (0.6, 0.0), (-0.5, 0.5), (-0.1, 0.0), (-0.5, -0.5),
].into();

// A closed L-shaped contour.
let outline: Path = vec![
    (1.0, 1.0), (5.0, 1.0), (5.0, 2.0),
    (2.5, 2.0), (2.5, 3.5), (1.0, 3.5),
].into();

let sum  = outline.minkowski_sum (pattern.clone(), true);
let diff = outline.minkowski_diff(pattern,         true);
```

| `minkowski_sum` | `minkowski_diff` |
|---|---|
| ![Result of the Minkowski sum example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/minkowski-sum.png) | ![Result of the Minkowski difference example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/minkowski-diff.png) |
| The arrowhead extends the swept boundary **rightward** (its tip is at `+0.6` x); the concave notch carves an inner ring on the **right** half of the L. | The same arrowhead, applied via difference, extends the swept boundary **leftward**; the concave notch now carves the inner ring on the **left** half of the L. |

## What's exposed

- Polygon boolean operations — intersection, union, difference, XOR — through a fluent `Clipper` builder or as standalone functions
- Polygon offsetting (inflate / deflate) with configurable corner styles (`JoinType`: square / bevel / round / miter) and endpoint styles (`EndType`: polygon / joined / butt / square / round)
- Minkowski sum and difference for sweeping an arbitrary polygonal kernel along a path or polygon (square / drag-knife / asymmetric tool footprints, configuration-space obstacles)
- Path simplification
- Point-in-polygon test
- Polygon area (signed and unsigned)
- Geometric transforms on `Path` / `Paths` — `translate`, `rotate`, `scale` (uniform or anisotropic, optionally around a point), `flip_x` / `flip_y`
- Path utilities — `append`, `closest_point`, `shift_start_to`, `surrounds_path`, `rectangle`, axis-aligned `Bounds`
- Optional `serde` support for `Point`, `Path`, `Paths`

## Typical use cases

- **CAD / CNC / 3D-printing slicers** — toolpath offsetting, pocketing, infill generation, contour boolean operations
- **GIS / mapping** — polygon overlay, buffer zones, vector tile clipping
- **Vector graphics & rendering** — path stroking via offset, SVG-style clipping, tessellation pre-pass
- **Game development** — visibility polygons, navigation mesh boolean operations, collision-region merging
- **Robotics / motion planning** — swept-area computation, configuration-space approximations
- **Computational geometry research** — robust boolean operations on polygons

## API uses `f64`, but `i64` under the hood

The Clipper2 library
[uses `i64` values](https://www.angusj.com/clipper2/Docs/Robustness.htm)
internally to guarantee the robustness of all calculations. This crate
takes `f64` coordinates at the API surface and rescales them to `i64`
for the C++ engine.

The types `Point`, `Path`, and `Paths` accept a `PointScaler` generic
parameter that controls the conversion factor. By default this is the
`Centi` scaler, which multiplies values by `100` — pick a larger
scaler if you need sub-centi precision, or a smaller one if your input
range is so wide that 100× would overflow.

## Building

The crate compiles the vendored Clipper2 C++ source through the
`clipper2c-sys` build, and therefore requires a working **C++17**
toolchain on the build host. CI exercises the default toolchains
shipped with each GitHub-hosted runner:

| Runner         | Default C++ toolchain          |
|----------------|--------------------------------|
| ubuntu-latest  | system `g++` / `clang++`       |
| macos-latest   | Xcode-shipped `clang++`        |
| windows-latest | MSVC (Visual Studio Build Tools) |

Optional features:

- `serde` — derives `Serialize` / `Deserialize` on `Point`, `Path`, and `Paths` (also enables the matching feature on `clipper2c-sys`).
- `doc-images` — embeds illustrative images in the rustdoc output (enabled by default).

WebAssembly: `wasm32-unknown-unknown` builds via the WASI SDK
toolchain — see `scripts/wasm-check.sh`.

## Status

This crate is pre-1.0; expect breaking changes between minor versions
as the API is refined. Suggestions on how the API can be simplified,
or direct code contributions, are welcome — see
[CONTRIBUTING.md](https://github.com/tirithen/clipper2/blob/main/CONTRIBUTING.md)
for more details.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/tirithen/clipper2/blob/main/LICENSE-APACHE.md)
or [MIT license](https://github.com/tirithen/clipper2/blob/main/LICENSE-MIT.md)
at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in clipper2 by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
