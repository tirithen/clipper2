mod clipper2;
mod boolean;

#[cfg(test)]
mod tests;

pub use clipper2::{Paths,Point};
pub use boolean::{union, FillRule};