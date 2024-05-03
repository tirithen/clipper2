use clipper2c_sys::{clipper_path64_of_points, clipper_path64_size, ClipperPath64, ClipperPoint64};

use crate::{malloc, Bounds, Centi, Point, PointScaler};

/// A collection of points.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let path_from_tuples: Path = vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)].into();
/// let path_from_slices: Path = vec![[0.0, 0.0], [5.0, 0.0], [5.0, 6.0], [0.0, 6.0]].into();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Path<P: PointScaler = Centi>(Vec<Point<P>>);

impl<P: PointScaler> Path<P> {
    /// Create a new path from a vector of points.
    pub fn new(points: Vec<Point<P>>) -> Self {
        Path(points)
    }

    /// Returns the number of points in the path.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the path contains at least one point
    pub fn contains_points(&self) -> bool {
        self.is_empty()
    }

    /// Returns an iterator over the points in the path.
    pub fn iter(&self) -> PathIterator<P> {
        PathIterator {
            items: self,
            index: 0,
        }
    }

    /// Construct a clone with each point offset with a x/y distance
    pub fn offset(&self, x: f64, y: f64) -> Self {
        Self::new(
            self.0
                .iter()
                .map(|p| Point::<P>::new(p.x() + x, p.y() + y))
                .collect(),
        )
    }

    /// Returns the bounds for this path.
    pub fn bounds(&self) -> Bounds {
        let mut bounds = Bounds::minmax();

        for p in &self.0 {
            let x = p.x();
            let y = p.y();

            if x < bounds.min.x() {
                bounds.min = Point::new(x, bounds.min.y());
            }

            if y < bounds.min.y() {
                bounds.min = Point::new(bounds.min.x(), y);
            }

            if x > bounds.max.x() {
                bounds.max = Point::new(x, bounds.max.y());
            }

            if y > bounds.max.y() {
                bounds.max = Point::new(bounds.max.x(), y);
            }
        }

        bounds
    }

    pub(crate) unsafe fn to_clipperpath64(&self) -> *mut ClipperPath64 {
        let mem = malloc(clipper_path64_size());
        clipper_path64_of_points(
            mem,
            self.0
                .iter()
                .cloned()
                .map(|point: Point<P>| ClipperPoint64 {
                    x: point.x_scaled(),
                    y: point.y_scaled(),
                })
                .collect::<Vec<_>>()
                .as_mut_ptr(),
            self.len(),
        )
    }
}

/// An iterator over the points in a path.
pub struct PathIterator<'a, P: PointScaler> {
    items: &'a Path<P>,
    index: usize,
}

impl<'a, P: PointScaler> Iterator for PathIterator<'a, P> {
    type Item = &'a Point<P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.0.len() {
            let result = Some(&self.items.0[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl<P: PointScaler> From<Path<P>> for Vec<Point<P>> {
    fn from(path: Path<P>) -> Self {
        path.0.clone()
    }
}

impl<P: PointScaler> From<Path<P>> for Vec<(f64, f64)> {
    fn from(path: Path<P>) -> Self {
        path.iter().map(|point| (point.x(), point.y())).collect()
    }
}

impl<P: PointScaler> From<Path<P>> for Vec<[f64; 2]> {
    fn from(path: Path<P>) -> Self {
        path.iter().map(|point| [point.x(), point.y()]).collect()
    }
}

impl<P: PointScaler> From<Vec<Point<P>>> for Path<P> {
    fn from(points: Vec<Point<P>>) -> Self {
        Path::new(points)
    }
}

impl<P: PointScaler> From<Vec<(f64, f64)>> for Path<P> {
    fn from(points: Vec<(f64, f64)>) -> Self {
        Path::<P>::new(points.iter().map(Point::<P>::from).collect())
    }
}

impl<P: PointScaler> From<Vec<[f64; 2]>> for Path<P> {
    fn from(points: Vec<[f64; 2]>) -> Self {
        Path::<P>::new(points.iter().map(Point::<P>::from).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        let path = Path::<Centi>::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let output: Vec<(f64, f64)> = path.into();
        assert_eq!(output, vec![(0.0, 0.0), (1.0, 1.0)]);
    }

    #[test]
    fn test_from_custom_scaler() {
        #[derive(Debug, Copy, Clone)]
        struct CustomScaler;

        impl PointScaler for CustomScaler {
            const MULTIPLIER: f64 = 1000.0;
        }

        let path = Path::<CustomScaler>::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let output: Vec<(f64, f64)> = path.clone().into();
        assert_eq!(output, vec![(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(path.0[0].x_scaled(), 0);
        assert_eq!(path.0[0].y_scaled(), 0);
        assert_eq!(path.0[1].x_scaled(), 1000);
        assert_eq!(path.0[1].y_scaled(), 1000);
    }
}
