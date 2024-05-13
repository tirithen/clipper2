#![warn(missing_docs)]

//! clipper2 is a path/polygon clipping and offsetting library that supports
//! operations like difference, inflate, intersect, point-in-polygon, union,
//! xor, simplify.
//!
//! The create uses the [clipper2c-sys](https://crates.io/crates/clipper2c-sys)
//! crate that in turn is a Rust wrapper around the C++ version of
//! [Clipper2](https://github.com/AngusJohnson/Clipper2).
//!
//! The crate exposes the Clipper API in two alternative ways until the best
//! version has been figured out.
//!
//! 1. Through the [`Path`]/[`Paths`] struct methods:
//!     * [`Path::inflate`]
//!     * [`Path::simplify`]
//!     * [`Path::is_point_inside`]
//!     * [`Paths::inflate`]
//!     * [`Paths::simplify`]
//!     * [`Paths::to_clipper_subject`] returns a [`Clipper`] builder struct
//!       with the current set of paths as the first subject, and allowing to
//!       make boolean operations on several sets of paths in one go.
//!     * [`Paths::to_clipper_open_subject`] similar but adds the current set
//!       of paths as an open "line" rather than a closed path/polygon.
//! 2. Via the plain functions:
//!     * [`difference`]
//!     * [`inflate`]
//!     * [`intersect`]
//!     * [`point_in_polygon`]
//!     * [`simplify`]
//!     * [`union`]
//!     * [`xor`]
//!
//! The [`Path`]/[`Paths`] structs also thas some transformation methods such
//! as:
//!
//! * [`Path::translate`] / [`Paths::translate`] for moving a path in by a x/y
//!   offset
//! * [`Path::rotate`] / [`Paths::rotate`] for rotating a path in by x radians
//! * [`Path::scale`] / [`Paths::scale`] for scaling a path by multiplier
//!
//! # Examples
//!
//! ```rust
//! use clipper2::*;
//!
//! let path_a: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
//! let path_b: Paths = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)].into();
//!
//! let output: Vec<Vec<(f64, f64)>> = path_a
//!     .to_clipper_subject()
//!     .add_clip(path_b)
//!     .difference(FillRule::default())
//!     .expect("Failed difference operation")
//!     .into();
//!
//! dbg!(output);
//! ```
//!
//! ![Image displaying the result of the difference example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/difference.png)
//!
//! More examples can be found in the
//! [examples](https://github.com/tirithen/clipper2/tree/main/examples)
//! directory.

mod bounds;
mod clipper;
mod operations;
mod options;
mod path;
mod paths;
mod point;

pub use crate::bounds::*;
pub use crate::clipper::*;
pub use crate::operations::*;
pub use crate::options::*;
pub use crate::path::*;
pub use crate::paths::*;
pub use crate::point::*;

pub(crate) unsafe fn malloc(size: usize) -> *mut std::os::raw::c_void {
    libc::malloc(size)
}
