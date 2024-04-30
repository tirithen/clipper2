use crate::Point;

/// Represents an area from one min and one max [Point](struct.Point.html).
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Bounds {
    /// Minimum point of the boundary.
    pub min: Point,
    /// Maximum point of the boundary.
    pub max: Point,
}

impl Bounds {
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
    pub fn size(&self) -> Point {
        Point::new(self.max.x() - self.min.x(), self.max.y() - self.min.y())
    }

    /// Return the center of the bounds area as a [Point](struct.Point.html).
    #[must_use]
    pub fn center(&self) -> Point {
        let size = self.size();
        Point::new(self.min.x() + size.x() / 2.0, self.min.y() + size.y() / 2.0)
    }
}
