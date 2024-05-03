use clipper2c_sys::{clipper_delete_paths64, clipper_paths64_simplify, clipper_paths64_size};

use crate::{malloc, Paths, PointScaler};

/// This function removes points that are less than the specified epsilon
/// distance from an imaginary line that passes through its 2 adjacent points.
/// Logically, smaller epsilon values will be less aggressive in removing
/// points than larger epsilon values.
///
/// This function is strongly recommended following offsetting
/// (ie inflating/shrinking), especially when offsetting paths multiple times.
/// Offsetting often creates tiny segments that don't improve path quality.
/// Further these tiny segments can be at angles that have been affected by
/// integer rounding. While these tiny segments are too small to be noticeable
/// following a single offset procedure, they can degrade both the shape quality
/// and the performance of subsequent offsets.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let path: Paths = vec![(1.0, 2.0), (1.0, 2.5), (1.2, 4.0), (1.8, 6.0)].into();
/// let path_simplified = simplify(path.offset(3.0, 0.0), 0.5, false);
///
/// dbg!(path, path_simplified);
/// ```
/// ![Image displaying the result of the simplify example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/simplify.png)
///
/// For more details see [simplify](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/SimplifyPaths.htm).
pub fn simplify<P: PointScaler>(paths: Paths<P>, epsilon: f64, is_open: bool) -> Paths<P> {
    let epsilon = P::scale(epsilon);

    unsafe {
        let mem = malloc(clipper_paths64_size());
        let paths_ptr = paths.to_clipperpaths64();
        let result_ptr = clipper_paths64_simplify(mem, paths_ptr, epsilon, is_open.into());
        clipper_delete_paths64(paths_ptr);
        let result = Paths::from_clipperpaths64(result_ptr);
        clipper_delete_paths64(result_ptr);
        result
    }
}

#[cfg(test)]
mod test {
    use crate::Centi;

    use super::*;

    #[test]
    fn test_simplify() {
        let path = vec![(0.0, 1.0), (0.1, 0.3), (1.0, 0.0), (1.3, 0.0), (2.0, 0.0)];
        let expected_output = vec![vec![(0.0, 1.0), (2.0, 0.0)]];

        let output: Vec<Vec<(f64, f64)>> = simplify::<Centi>(path.into(), 1.0, true).into();
        assert_eq!(output, expected_output);
    }
}
