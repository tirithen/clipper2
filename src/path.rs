use clipper2c_sys::{
    clipper_delete_path64, clipper_path64_area, clipper_path64_get_point, clipper_path64_length,
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
#[derive(Debug, Clone, Default, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(bound = "P: PointScaler")
)]
pub struct Path<P: PointScaler = Centi>(Vec<Point<P>>);

impl<P: PointScaler> Eq for Path<P> {}

impl<P: PointScaler> Path<P> {
    /// Create a new path from a vector of points.
    pub fn new(points: Vec<Point<P>>) -> Self {
        Path(points)
    }

    /// In place push point onto this path.
    pub fn push(&mut self, point: impl Into<Point<P>>) {
        self.0.push(point.into());
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

    /// Creates a path in a rectangle shape
    pub fn rectangle(x: f64, y: f64, size_x: f64, size_y: f64) -> Self {
        vec![
            (x, y),
            (x + size_x, y),
            (x + size_x, y + size_y),
            (x, y + size_y),
        ]
        .into()
    }

    /// Returns an iterator over the points in the path.
    pub fn iter(&self) -> std::slice::Iter<'_, Point<P>> {
        self.0.iter()
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::Path;
    /// let path: Path = vec![(-1.0, -1.0), (1.0, 1.0)].into();
    /// let scaled = path.scale(2.0, 2.0);
    /// assert_eq!(scaled.iter().map(|p| (p.x(), p.y())).collect::<Vec<_>>(), vec![(-2.0, -2.0), (2.0, 2.0)]);
    /// ```
    pub fn scale(&self, scale_x: f64, scale_y: f64) -> Self {
        let center = self.bounds().center();
        self.scale_around_point(scale_x, scale_y, center)
    }

    /// Construct a scaled clone of the path with the origin at a given point
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::Path;
    /// let path: Path = vec![(0.0, 0.0), (1.0, 1.0)].into();
    /// let scaled = path.scale_around_point(2.0, 2.0, (0.0, 0.0).into());
    /// assert_eq!(scaled.iter().map(|p| (p.x(), p.y())).collect::<Vec<_>>(), vec![(0.0, 0.0), (2.0, 2.0)]);
    /// ```
    pub fn scale_around_point(&self, scale_x: f64, scale_y: f64, point: Point<P>) -> Self {
        Self::new(
            self.0
                .iter()
                .map(|p| {
                    Point::<P>::new(
                        (p.x() - point.x()) * scale_x + point.x(),
                        (p.y() - point.y()) * scale_y + point.y(),
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
    pub fn bounds(&self) -> Bounds<P> {
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
    /// For closed paths passing a positive delta number will inflate the path
    /// where passing a negative number will shrink the path.
    ///
    /// **NOTE:** Inflate calls will frequently generate a large amount of very
    /// close extra points and it is therefore recommented to almost always call
    /// [`Path::simplify`] on the path after inflating/deflating it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Path = vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)].into();
    /// let inflated = path
    ///     .inflate(1.0, JoinType::Square, EndType::Polygon, 2.0)
    ///     .simplify(0.01, false);
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

    /// This function returns the area of the supplied polygon. It's assumed
    /// that the path is closed and does not self-intersect.
    ///
    /// Depending on the path's winding orientation, this value may be positive
    /// or negative. Assuming paths are displayed in a Cartesian plane (with X
    /// values increasing heading right and Y values increasing heading up) then
    /// clockwise winding will have negative areas and counter-clockwise winding
    /// have positive areas.
    ///
    /// Conversely, when paths are displayed where Y values increase heading
    /// down, then clockwise paths will have positive areas, and
    /// counter-clockwise paths will have negative areas.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Path = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)].into();
    ///
    /// assert_eq!(path.signed_area(), 1.0);
    /// ```
    ///
    pub fn signed_area(&self) -> f64 {
        unsafe { clipper_path64_area(self.to_clipperpath64()) / (P::MULTIPLIER * P::MULTIPLIER) }
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
                    ..Default::default()
                })
                .collect::<Vec<_>>()
                .as_mut_ptr(),
            self.len(),
        )
    }
}

impl<P: PointScaler> IntoIterator for Path<P> {
    type Item = Point<P>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<P: PointScaler> FromIterator<Point<P>> for Path<P> {
    fn from_iter<T: IntoIterator<Item = Point<P>>>(iter: T) -> Self {
        Path(iter.into_iter().collect())
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
        #[derive(Debug, Clone, Copy, PartialEq, Hash)]
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

    #[test]
    fn test_into_iterator() {
        let path = Path::<Centi>::from(vec![(0.0, 0.0), (1.0, 1.0)]);

        let mut count = 0;

        for point in path {
            assert_eq!(point.x(), point.y());
            count += 1;
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn test_iter() {
        let path = Path::<Centi>::from(vec![(0.0, 0.0), (1.0, 1.0)]);

        let mut count = 0;

        for point in path.iter() {
            assert_eq!(point.x(), point.y());
            count += 1;
        }

        assert_eq!(count, 2);

        let x_values: Vec<_> = path.iter().map(|point| point.x()).collect();
        assert_eq!(x_values, vec![0.0, 1.0]);
    }

    #[test]
    fn test_into_iter() {
        let path = Path::<Centi>::from(vec![(0.0, 0.0), (1.0, 1.0)]);

        let mut count = 0;

        for point in path.clone().into_iter() {
            assert_eq!(point.x(), point.y());
            count += 1;
        }

        assert_eq!(count, 2);

        let x_values: Vec<_> = path.into_iter().map(|point| point.x()).collect();
        assert_eq!(x_values, vec![0.0, 1.0]);
    }

    #[test]
    fn test_signed_area() {
        let path = Path::<Centi>::rectangle(10.0, 20.0, 30.0, 15.0);
        let area = path.signed_area();
        assert_eq!(area, 450.0);
    }

    #[test]
    fn test_signed_area_negative() {
        let path = Path::<Centi>::rectangle(-20.0, 25.0, -40.0, 30.0);
        let area = path.signed_area();
        assert_eq!(area, -1200.0);
    }

    #[cfg(all(feature = "serde", not(feature = "usingz")))]
    #[test]
    fn test_serde() {
        let path = Path::<Centi>::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let serialized = serde_json::to_string(&path).unwrap();
        assert_eq!(serialized, r#"[{"x":0,"y":0},{"x":100,"y":100}]"#);

        let deserialized: Path = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, path);
    }
}
