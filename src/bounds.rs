use crate::{Centi, Point, PointScaler};

/// Represents an area from one min and one max [Point](struct.Point.html).
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Bounds<P: PointScaler = Centi> {
    /// Minimum point of the boundary.
    pub min: Point<P>,
    /// Maximum point of the boundary.
    pub max: Point<P>,
}

impl<P: PointScaler> Bounds<P> {
    /// Create a `Bounds` struct starting at xy 0.0 and ending at the given xy
    /// coordinates.
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            min: Point::new(0.0, 0.0),
            max: Point::new(x, y),
        }
    }

    /// Create a `Bounds` struct using xy maximum value as min and minimum value
    /// for max.
    #[must_use]
    pub fn minmax() -> Self {
        Self {
            min: Point::MAX,
            max: Point::MIN,
        }
    }

    /// Return the size of the bounds area as a [Point](struct.Point.html).
    #[must_use]
    pub fn size(&self) -> Point<P> {
        Point::new(self.max.x() - self.min.x(), self.max.y() - self.min.y())
    }

    /// Return the center of the bounds area as a [Point](struct.Point.html).
    #[must_use]
    pub fn center(&self) -> Point<P> {
        let size = self.size();
        Point::new(self.min.x() + size.x() / 2.0, self.min.y() + size.y() / 2.0)
    }
}
