#![warn(missing_docs)]

//! clipper2 is a path/polygon clipping and offsetting library that supports
//! operations like difference, inflate, intersect, point-in-polygon, union,
//! xor, simplify.
//!
//! The create uses the [clipper2c-sys](https://crates.io/crates/clipper2c-sys)
//! crate that in turn is a Rust wrapper around the C++ version of
//! [Clipper2](https://github.com/AngusJohnson/Clipper2).
//!
//! The operations that are currently exposed are:
//!
//! * [difference](difference())
//! * [inflate](inflate())
//! * [intersect](intersect())
//! * [point_in_polygon](point_in_polygon())
//! * [union](union())
//! * [xor](xor())
//!
//! # Examples
//!
//! ```rust
//! use clipper2::*;
//!
//! let path_a: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
//! let path_b: Paths = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)].into();
//!
//! let output: Vec<Vec<(f64, f64)>> = difference(path_a, path_b, FillRule::default())
//!     .expect("Failed to run boolean operation").into();
//!
//! dbg!(output);
//! ```
//!
//! ![Image displaying the result of the difference example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/difference.png)
//!
//! More examples can be found in the
//! [examples](https://github.com/tirithen/clipper2/tree/main/examples)
//! directory.

mod clipper;
mod operations;
mod options;
mod path;
mod paths;
mod point;

pub use crate::clipper::*;
pub use crate::operations::*;
pub use crate::options::*;
pub use crate::path::*;
pub use crate::paths::*;
pub use crate::point::*;

pub(crate) unsafe fn malloc(size: usize) -> *mut std::os::raw::c_void {
    libc::malloc(size)
}
