use clipper2c_sys::{
    clipper_delete_path64, clipper_path64_get_point, clipper_path64_length,
    clipper_path64_of_points, clipper_path64_simplify, clipper_path64_size, ClipperPath64,
    ClipperPoint64,
};

use crate::{
    inflate, malloc, point_in_polygon, Bounds, Centi, EndType, JoinType, Point,
    PointInPolygonResult, PointScaler,
};

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

    /// Construct a clone with each point offset by a x/y distance
    #[deprecated]
    pub fn offset(&self, x: f64, y: f64) -> Self {
        self.translate(x, y)
    }

    /// Construct a clone with each point offset by a x/y distance
    pub fn translate(&self, x: f64, y: f64) -> Self {
        Self::new(
            self.0
                .iter()
                .map(|p| Point::<P>::new(p.x() + x, p.y() + y))
                .collect(),
        )
    }

    /// Construct a scaled clone of the path with the origin at the path center
    pub fn scale(&self, scale: f64) -> Self {
        let bounds = self.bounds();
        let center = bounds.center();

        Self::new(
            self.0
                .iter()
                .map(|p| {
                    Point::<P>::new(
                        (center.x() - p.x()) * scale + center.x(),
                        (center.y() - p.y()) * scale + center.y(),
                    )
                })
                .collect(),
        )
    }

    /// Construct a rotated clone of the path with the origin at the path center
    pub fn rotate(&self, radians: f64) -> Self {
        let bounds = self.bounds();
        let center = bounds.center();
        let cos = radians.cos();
        let sin = radians.sin();

        Self::new(
            self.0
                .iter()
                .map(|p| {
                    Point::<P>::new(
                        (center.x() - p.x()) * cos - (center.y() - p.y()) * sin + center.x(),
                        (center.x() - p.x()) * sin + (center.y() - p.y()) * cos + center.y(),
                    )
                })
                .collect(),
        )
    }

    /// Construct a clone with each point x value flipped
    pub fn flip_x(&self) -> Self {
        let bounds = self.bounds();
        let center = bounds.center();

        Self::new(
            self.0
                .iter()
                .map(|p| Point::<P>::new(center.x() + (center.x() - p.x()), p.y()))
                .collect(),
        )
    }

    /// Construct a clone with each point y value flipped
    pub fn flip_y(&self) -> Self {
        let bounds = self.bounds();
        let center = bounds.center();

        Self::new(
            self.0
                .iter()
                .map(|p| Point::<P>::new(p.x(), center.y() + (center.y() - p.y())))
                .collect(),
        )
    }

    /// Returns the bounds for this path
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

    /// Construct a new path offset from this one by a delta distance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Path = vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)].into();
    /// let inflated = path.inflate(1.0, JoinType::Square, EndType::Polygon, 2.0);
    /// ```
    ///
    /// For more details see the original [inflate paths](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/InflatePaths.htm) docs.
    pub fn inflate(
        &self,
        delta: f64,
        join_type: JoinType,
        end_type: EndType,
        miter_limit: f64,
    ) -> Self {
        inflate(self.clone(), delta, join_type, end_type, miter_limit)
            .iter()
            .next()
            .unwrap()
            .clone()
    }

    /// Construct a new path from this one but with a reduced set of points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Path = vec![(0.0, 0.0), (5.0, 0.002), (5.0, 0.01), (5.1, 0.0), (5.0, 6.0), (0.0, 6.0)].into();
    /// let simplified = path.simplify(1.0, true);
    /// ```
    ///
    /// For more details see the original [simplify](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/SimplifyPaths.htm) docs.
    pub fn simplify(&self, epsilon: f64, is_open: bool) -> Self {
        let epsilon = P::scale(epsilon);

        unsafe {
            let mem = malloc(clipper_path64_size());
            let paths_ptr = self.to_clipperpath64();
            let result_ptr = clipper_path64_simplify(mem, paths_ptr, epsilon, is_open.into());
            clipper_delete_path64(paths_ptr);
            let result = Path::from_clipperpath64(result_ptr);
            clipper_delete_path64(result_ptr);
            result
        }
    }

    /// The function result indicates whether the point is inside, or outside,
    /// or on one of the edges edges of this path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Path = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)].into();
    ///
    /// let output = path.is_point_inside(Point::new(0.5, 0.5));
    ///
    /// dbg!(output);
    /// ```
    ///
    /// For more details see the original [point-in-polygon](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/PointInPolygon.htm) docs.
    pub fn is_point_inside(&self, point: Point<P>) -> PointInPolygonResult {
        point_in_polygon(point, self)
    }

    pub(crate) fn from_clipperpath64(ptr: *mut ClipperPath64) -> Self {
        let paths = unsafe {
            let len: i32 = clipper_path64_length(ptr).try_into().unwrap();
            (0..len)
                .map(|i| clipper_path64_get_point(ptr, i).into())
                .collect()
        };
        Self::new(paths)
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
