use clipper2c_sys::{
    clipper_delete_path64, clipper_paths64_get_point, clipper_paths64_length,
    clipper_paths64_of_paths, clipper_paths64_path_length, clipper_paths64_size, ClipperPath64,
    ClipperPaths64,
};

use crate::{malloc, Bounds, Centi, Path, Point, PointScaler};

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
#[derive(Debug, Clone, Default)]
pub struct Paths<P: PointScaler = Centi>(Vec<Path<P>>);

impl<P: PointScaler> Paths<P> {
    /// Create a new paths from a vector of paths.
    pub fn new(paths: Vec<Path<P>>) -> Self {
        Paths(paths)
    }

    /// Returns the number of paths.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no paths added.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if at least one of the paths contains a point
    pub fn contains_points(&self) -> bool {
        for path in &self.0 {
            if !path.is_empty() {
                return true;
            }
        }

        false
    }

    /// Returns an iterator over the paths in the paths.
    pub fn iter(&self) -> PathsIterator<P> {
        PathsIterator {
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
        Self::new(self.0.iter().map(|p| p.translate(x, y)).collect())
    }

    /// Construct a clone with each point x value flipped
    pub fn flip_x(&self) -> Self {
        Self::new(self.0.iter().map(|p| p.flip_x()).collect())
    }

    /// Construct a clone with each point y value flipped
    pub fn flip_y(&self) -> Self {
        Self::new(self.0.iter().map(|p| p.flip_y()).collect())
    }

    /// Returns the bounds for this path.
    pub fn bounds(&self) -> Bounds {
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

/// An iterator over the paths.
pub struct PathsIterator<'a, P: PointScaler> {
    items: &'a Paths<P>,
    index: usize,
}

impl<'a, P: PointScaler> Iterator for PathsIterator<'a, P> {
    type Item = &'a Path<P>;

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
        #[derive(Debug, Copy, Clone)]
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
}
