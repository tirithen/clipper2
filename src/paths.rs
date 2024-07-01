use clipper2c_sys::{
    clipper_delete_path64, clipper_paths64_area, clipper_paths64_get_point, clipper_paths64_length,
    clipper_paths64_of_paths, clipper_paths64_path_length, clipper_paths64_size, ClipperPath64,
    ClipperPaths64,
};

use crate::{
    inflate, malloc, simplify, Bounds, Centi, Clipper, EndType, JoinType, Path, Point, PointScaler,
    WithSubjects,
};

/// A collection of paths.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let paths_from_single_vec: Paths = vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)].into();
/// let paths_from_vec_of_vecs: Paths = vec![vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)]].into();
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(bound = "P: PointScaler")
)]
pub struct Paths<P: PointScaler = Centi>(Vec<Path<P>>);

impl<P: PointScaler> Paths<P> {
    /// Create a new paths from a vector of paths.
    pub fn new(paths: Vec<Path<P>>) -> Self {
        Paths(paths)
    }

    /// In place push paths onto this set of paths.
    pub fn push(&mut self, paths: impl Into<Paths<P>>) {
        for path in paths.into() {
            self.0.push(path);
        }
    }

    /// Returns the number of paths.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no paths added.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if at least one of the paths contains a point.
    pub fn contains_points(&self) -> bool {
        for path in &self.0 {
            if !path.is_empty() {
                return true;
            }
        }

        false
    }

    /// Returns a reference to the first path in the set of paths wrapped in an
    /// option.
    pub fn first(&self) -> Option<&Path<P>> {
        self.iter().next()
    }

    /// Returns a reference to the path at the given index in the set of paths
    /// wrapped in an option.
    pub fn get(&self, index: usize) -> Option<&Path<P>> {
        self.0.get(index)
    }

    /// Returns an iterator over the paths in the paths.
    pub fn iter(&self) -> std::slice::Iter<'_, Path<P>> {
        self.0.iter()
    }

    /// Construct a clone with each point offset by a x/y distance.
    pub fn translate(&self, x: f64, y: f64) -> Self {
        Self::new(self.0.iter().map(|p| p.translate(x, y)).collect())
    }

    /// Construct a scaled clone of the path with the origin at the path center.
    pub fn scale(&self, scale_x: f64, scale_y: f64) -> Self {
        let center = self.bounds().center();
        self.scale_around_point(scale_x, scale_y, center)
    }

    /// Construct a scaled clone of the path with the origin at a given point.
    pub fn scale_around_point(&self, scale_x: f64, scale_y: f64, point: Point<P>) -> Self {
        Self::new(
            self.0
                .iter()
                .map(|p| p.scale_around_point(scale_x, scale_y, point))
                .collect(),
        )
    }

    /// Construct a rotated clone of the path with the origin at the path
    /// center.
    pub fn rotate(&self, radians: f64) -> Self {
        Self::new(self.0.iter().map(|p| p.rotate(radians)).collect())
    }

    /// Construct a clone with each point x value flipped.
    pub fn flip_x(&self) -> Self {
        Self::new(self.0.iter().map(|p| p.flip_x()).collect())
    }

    /// Construct a clone with each point y value flipped.
    pub fn flip_y(&self) -> Self {
        Self::new(self.0.iter().map(|p| p.flip_y()).collect())
    }

    /// Returns the bounds for this path.
    pub fn bounds(&self) -> Bounds<P> {
        let mut bounds = Bounds::minmax();

        for p in &self.0 {
            let b = p.bounds();
            let min_x = b.min.x();
            let min_y = b.min.y();
            let max_x = b.max.x();
            let max_y = b.max.y();

            if min_x < bounds.min.x() {
                bounds.min = Point::new(min_x, bounds.min.y());
            }

            if min_y < bounds.min.y() {
                bounds.min = Point::new(bounds.min.x(), min_y);
            }

            if max_x > bounds.max.x() {
                bounds.max = Point::new(max_x, bounds.max.y());
            }

            if max_y > bounds.max.y() {
                bounds.max = Point::new(bounds.max.x(), max_y);
            }
        }

        bounds
    }

    /// Construct a new set of paths offset from this one by a delta distance.
    ///
    /// For closed paths passing a positive delta number will inflate the path
    /// where passing a negative number will shrink the path.
    ///
    /// **NOTE:** Inflate calls will frequently generate a large amount of very
    /// close extra points and it is therefore recommented to almost always call
    /// [`Paths::simplify`] on the path after inflating/shrinking it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let paths: Paths = vec![vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)]].into();
    /// let inflated = paths
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
    }

    /// Construct a new set of paths from these ones but with a reduced set of
    /// points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let paths: Paths = vec![vec![(0.0, 0.0), (5.0, 0.002), (5.0, 0.01), (5.1, 0.0), (5.0, 6.0), (0.0, 6.0)]].into();
    /// let simplified = paths.simplify(1.0, true);
    /// ```
    ///
    /// For more details see the original [simplify](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/SimplifyPaths.htm) docs.
    pub fn simplify(&self, epsilon: f64, is_open: bool) -> Self {
        simplify(self.clone(), epsilon, is_open)
    }

    /// Create a [`Clipper`] builder with this set of paths as the subject that
    /// will allow for making boolean operations on this set of paths.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![vec![(0.0, 0.0), (5.0, 6.0), (0.0, 6.0)]].into();
    /// let path2: Paths = vec![vec![(1.0, 1.0), (4.0, 1.0), (1.0, 4.0)]].into();
    /// let result = path.to_clipper_subject().add_clip(path2).union(FillRule::default());
    /// ```
    pub fn to_clipper_subject(&self) -> Clipper<WithSubjects, P> {
        let clipper = Clipper::new();
        clipper.add_subject(self.clone())
    }

    /// Create a [`Clipper`] builder with this set of paths as the open subject
    /// that will allow for making boolean operations on this set of paths.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths =  vec![vec![(0.0, 0.0), (5.0, 6.0), (0.0, 6.0)]].into();
    /// let path2: Paths = vec![vec![(1.0, 1.0), (4.0, 1.0), (1.0, 4.0)]].into();
    /// let result = path.to_clipper_open_subject().add_clip(path2).difference(FillRule::default());
    /// ```
    pub fn to_clipper_open_subject(&self) -> Clipper<WithSubjects, P> {
        let clipper = Clipper::new();
        clipper.add_open_subject(self.clone())
    }

    /// This function returns the area of the supplied paths. It's assumed
    /// that the paths are closed and do not self-intersect.
    ///
    /// Depending on the paths' winding orientations, this value may be positive
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
    /// let paths: Paths = vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]].into();
    ///
    /// assert_eq!(paths.signed_area(), 1.0);
    /// ```
    ///
    pub fn signed_area(&self) -> f64 {
        unsafe { clipper_paths64_area(self.to_clipperpaths64()) / (P::MULTIPLIER * P::MULTIPLIER) }
    }

    pub(crate) fn from_clipperpaths64(ptr: *mut ClipperPaths64) -> Self {
        let paths = unsafe {
            let len: i32 = clipper_paths64_length(ptr).try_into().unwrap();
            (0..len)
                .map(|i| {
                    let point_len: i32 = clipper_paths64_path_length(ptr, i).try_into().unwrap();
                    let points = (0..point_len)
                        .map(|j| clipper_paths64_get_point(ptr, i, j).into())
                        .collect();
                    Path::new(points)
                })
                .collect()
        };
        Self::new(paths)
    }

    pub(crate) unsafe fn to_clipperpaths64(&self) -> *mut ClipperPaths64 {
        let mem = malloc(clipper_paths64_size());
        let mut paths = self
            .iter()
            .map(|p| p.to_clipperpath64())
            .collect::<Vec<*mut ClipperPath64>>();

        let result = clipper_paths64_of_paths(mem, paths.as_mut_ptr(), self.len());

        for path in paths {
            clipper_delete_path64(path);
        }

        result
    }
}

impl<'a, P: PointScaler> IntoIterator for &'a Path<P> {
    type Item = &'a Point<P>;
    type IntoIter = std::slice::Iter<'a, Point<P>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<P: PointScaler> IntoIterator for Paths<P> {
    type Item = Path<P>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<P: PointScaler> FromIterator<Path<P>> for Paths<P> {
    fn from_iter<T: IntoIterator<Item = Path<P>>>(iter: T) -> Self {
        Paths(iter.into_iter().collect())
    }
}

impl<P: PointScaler> From<Path<P>> for Paths<P> {
    fn from(path: Path<P>) -> Self {
        vec![path].into()
    }
}

impl<P: PointScaler> From<Paths<P>> for Vec<Path<P>> {
    fn from(paths: Paths<P>) -> Self {
        paths.0.clone()
    }
}

impl<P: PointScaler> From<Paths<P>> for Vec<Vec<(f64, f64)>> {
    fn from(paths: Paths<P>) -> Self {
        paths
            .iter()
            .map(|path| path.iter().map(|point| (point.x(), point.y())).collect())
            .collect()
    }
}

impl<P: PointScaler> From<Paths<P>> for Vec<Vec<[f64; 2]>> {
    fn from(paths: Paths<P>) -> Self {
        paths
            .iter()
            .map(|path| path.iter().map(|point| [point.x(), point.y()]).collect())
            .collect()
    }
}

impl<P: PointScaler> From<Vec<Vec<Point<P>>>> for Paths<P> {
    fn from(points: Vec<Vec<Point<P>>>) -> Self {
        Paths::<P>::new(points.into_iter().map(|path| path.into()).collect())
    }
}

impl<P: PointScaler> From<Vec<Vec<(f64, f64)>>> for Paths<P> {
    fn from(points: Vec<Vec<(f64, f64)>>) -> Self {
        Paths::<P>::new(points.into_iter().map(|path| path.into()).collect())
    }
}

impl<P: PointScaler> From<Vec<Vec<[f64; 2]>>> for Paths<P> {
    fn from(points: Vec<Vec<[f64; 2]>>) -> Self {
        Paths::<P>::new(points.into_iter().map(|path| path.into()).collect())
    }
}

impl<P: PointScaler> From<Vec<Point<P>>> for Paths<P> {
    fn from(points: Vec<Point<P>>) -> Self {
        Paths::<P>::new(vec![points.into()])
    }
}

impl<P: PointScaler> From<Vec<(f64, f64)>> for Paths<P> {
    fn from(points: Vec<(f64, f64)>) -> Self {
        Paths::<P>::new(vec![points.into()])
    }
}

impl<P: PointScaler> From<Vec<[f64; 2]>> for Paths<P> {
    fn from(points: Vec<[f64; 2]>) -> Self {
        Paths::<P>::new(vec![points.into()])
    }
}

impl<P: PointScaler> From<Vec<Path<P>>> for Paths<P> {
    fn from(points: Vec<Path<P>>) -> Self {
        Paths::<P>::new(points)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        let paths = Paths::<Centi>::from(vec![(0.4, 0.0), (5.0, 1.0)]);
        let output: Vec<Vec<(f64, f64)>> = paths.clone().into();
        assert_eq!(output, vec![vec![(0.4, 0.0), (5.0, 1.0)]]);

        let mut path_iter = paths.iter().next().unwrap().iter();
        let point1 = path_iter.next().unwrap();
        let point2 = path_iter.next().unwrap();
        assert_eq!(point1.x_scaled(), 40);
        assert_eq!(point1.y_scaled(), 0);
        assert_eq!(point2.x_scaled(), 500);
        assert_eq!(point2.y_scaled(), 100);
    }

    #[test]
    fn test_from_custom_scaler() {
        #[derive(Debug, Clone, Copy, PartialEq, Hash)]
        struct CustomScaler;

        impl PointScaler for CustomScaler {
            const MULTIPLIER: f64 = 1000.0;
        }

        let paths = Paths::<CustomScaler>::from(vec![(0.0, 0.6), (1.0, 2.0)]);
        let output: Vec<Vec<(f64, f64)>> = paths.clone().into();
        assert_eq!(output, vec![vec![(0.0, 0.6), (1.0, 2.0)]]);

        let mut path_iter = paths.iter().next().unwrap().iter();
        let point1 = path_iter.next().unwrap();
        let point2 = path_iter.next().unwrap();
        assert_eq!(point1.x_scaled(), 0);
        assert_eq!(point1.y_scaled(), 600);
        assert_eq!(point2.x_scaled(), 1000);
        assert_eq!(point2.y_scaled(), 2000);
    }

    #[test]
    fn test_into_iterator() {
        let paths = Paths::<Centi>::from(vec![vec![(0.0, 0.0), (1.0, 1.0)]; 2]);
        for path in paths {
            assert_eq!(path.len(), 2);
        }
    }

    #[test]
    fn test_iter() {
        let paths = Paths::<Centi>::from(vec![vec![(0.0, 0.0), (1.0, 1.0)]; 2]);

        let mut paths_iterator = paths.iter();
        assert_eq!(
            paths_iterator.next(),
            Some(&Path::from(vec![(0.0, 0.0), (1.0, 1.0)]))
        );
        assert_eq!(
            paths_iterator.next(),
            Some(&Path::from(vec![(0.0, 0.0), (1.0, 1.0)]))
        );
        assert_eq!(paths_iterator.next(), None);

        let x_values: Vec<_> = paths.iter().flatten().map(|point| point.x()).collect();
        assert_eq!(x_values, vec![0.0, 1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_into_iter() {
        let paths = Paths::<Centi>::from(vec![vec![(0.0, 0.0), (1.0, 1.0)]; 2]);

        let mut paths_iterator = paths.iter();
        assert_eq!(
            paths_iterator.next(),
            Some(&Path::from(vec![(0.0, 0.0), (1.0, 1.0)]))
        );
        assert_eq!(
            paths_iterator.next(),
            Some(&Path::from(vec![(0.0, 0.0), (1.0, 1.0)]))
        );
        assert_eq!(paths_iterator.next(), None);

        let x_values: Vec<_> = paths.into_iter().flatten().map(|point| point.x()).collect();
        assert_eq!(x_values, vec![0.0, 1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_signed_area() {
        let paths = Paths::new(vec![
            Path::<Centi>::rectangle(10.0, 20.0, 30.0, 150.0),
            Path::<Centi>::rectangle(40.0, 20.0, 10.0, 15.0),
        ]);
        let area = paths.signed_area();
        assert_eq!(area, 4650.0);
    }

    #[test]
    fn test_signed_area_negative() {
        let paths = Paths::new(vec![
            Path::<Centi>::rectangle(-20.0, 25.0, -45.0, 30.0),
            Path::<Centi>::rectangle(-20.0, 55.0, 15.0, 15.0),
        ]);
        let area = paths.signed_area();
        assert_eq!(area, -1125.0);
    }

    #[test]
    fn test_signed_area_counts_overlapping_areas_comulatively_for_each_path() {
        let paths = Paths::new(vec![
            Path::<Centi>::rectangle(10.0, 20.0, 30.0, 150.0),
            Path::<Centi>::rectangle(10.0, 20.0, 100.0, 15.0),
        ]);
        let area = paths.signed_area();
        assert_eq!(area, 6000.0);
    }

    #[test]
    fn test_scale_two_separate_triangles() {
        let paths = Paths::<Centi>::from(vec![
            vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
            vec![(10.0, 10.0), (11.0, 10.0), (10.0, 11.0)],
        ]);

        let scaled = paths.scale(4.0, 2.0);

        let expected_output = Paths::from(vec![
            vec![(-16.5, -5.5), (-12.5, -5.5), (-16.5, -3.5)],
            vec![(23.5, 14.5), (27.5, 14.5), (23.5, 16.5)],
        ]);

        assert_eq!(scaled, expected_output);
    }

    #[test]
    fn test_scale_overlapping_rectangles() {
        let paths = Paths::<Centi>::from(vec![
            Path::rectangle(-10.0, -20.0, 20.0, 40.0),
            Path::rectangle(-20.0, -10.0, 40.0, 20.0),
        ]);
        let scaled = paths.scale(4.0, 2.0);

        let expected_output = Paths::from(vec![
            vec![(-40.0, -40.0), (40.0, -40.0), (40.0, 40.0), (-40.0, 40.0)],
            vec![(-80.0, -20.0), (80.0, -20.0), (80.0, 20.0), (-80.0, 20.0)],
        ]);

        assert_eq!(scaled, expected_output);
    }

    #[test]
    fn test_scale_around_point() {
        let paths = Paths::<Centi>::from(vec![
            Path::rectangle(-10.0, -20.0, 20.0, 40.0),
            Path::rectangle(-20.0, -10.0, 40.0, 20.0),
        ]);

        let scaled = paths.scale_around_point(4.0, 2.0, Point::new(-10.0, -20.0));

        let expected_output = Paths::from(vec![
            vec![(-10.0, -20.0), (70.0, -20.0), (70.0, 60.0), (-10.0, 60.0)],
            vec![(-50.0, 0.0), (110.0, 0.0), (110.0, 40.0), (-50.0, 40.0)],
        ]);

        assert_eq!(scaled, expected_output);
    }

    #[test]
    fn test_from_iterator() {
        let paths = vec![
            Path::rectangle(-10.0, -20.0, 20.0, 40.0),
            Path::rectangle(-20.0, -10.0, 40.0, 20.0),
        ]
        .into_iter()
        .collect::<Paths>();

        assert_eq!(paths.len(), 2);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let paths = Paths::<Centi>::from(vec![(0.4, 0.0), (5.0, 1.0)]);
        let serialized = serde_json::to_string(&paths).unwrap();
        assert_eq!(serialized, r#"[[{"x":40,"y":0},{"x":500,"y":100}]]"#);

        let deserialized: Paths = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, paths);
    }
}
