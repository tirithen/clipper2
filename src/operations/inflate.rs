use clipper2c_sys::{clipper_delete_paths64, clipper_paths64_inflate, clipper_paths64_size};

use crate::{malloc, EndType, JoinType, Paths, PointScaler};

/// These functions encapsulate ClipperOffset, the class that performs both
/// polygon and open path offsetting.
///
/// # Example
///
/// ```rust
/// use clipper2::*;
///
/// let paths: Paths = vec![(2.0, 2.0), (6.0, 2.0), (6.0, 10.0), (2.0, 6.0)].into();
///
/// let output = inflate(paths, 1.0, JoinType::Round, EndType::Polygon, 0.0);
///
/// dbg!(output);
/// ```
///
/// ![Image displaying the result of the inflate example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/inflate.png)
///
/// For more details see the original [inflate paths](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/InflatePaths.htm) docs.
pub fn inflate<P: PointScaler>(
    paths: impl Into<Paths<P>>,
    delta: f64,
    join_type: JoinType,
    end_type: EndType,
    miter_limit: f64,
) -> Paths<P> {
    let delta = P::scale(delta);
    let miter_limit = P::scale(miter_limit);
    let paths: Paths<P> = paths.into();

    unsafe {
        let mem = malloc(clipper_paths64_size());
        let paths_ptr = paths.to_clipperpaths64();
        let result_ptr = clipper_paths64_inflate(
            mem,
            paths_ptr,
            delta,
            join_type.into(),
            end_type.into(),
            miter_limit,
        );
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
    fn test_inflate() {
        let paths = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let expected_output = vec![vec![
            (2.0, -0.41),
            (2.0, 1.41),
            (1.41, 2.0),
            (-0.41, 2.0),
            (-1.0, 1.41),
            (-1.0, -0.41),
            (-0.41, -1.0),
            (1.41, -1.0),
        ]];

        let output: Vec<Vec<(f64, f64)>> =
            inflate::<Centi>(paths, 1.0, JoinType::Square, EndType::Polygon, 0.0).into();
        assert_eq!(output, expected_output);
    }
}
