mod boolean;
mod clipper2;

#[cfg(test)]
mod tests;

pub use boolean::{union, FillRule};
pub use clipper2::{Paths, Point};
