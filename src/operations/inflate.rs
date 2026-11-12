use clipper2c_sys::{
    clipper_clipperoffset, clipper_clipperoffset_add_paths64, clipper_clipperoffset_execute,
    clipper_clipperoffset_size, clipper_delete_clipperoffset, clipper_delete_paths64,
    clipper_paths64_inflate, clipper_paths64_size,
};

use crate::{EndType, JoinType, Paths, PointScaler, malloc};

/// This function performs both closed path and open path offsetting.
///
/// For closed paths passing a positive delta number will inflate the path
/// where passing a negative number will shrink the path.
///
/// **NOTE:** Inflate calls will frequently generate a large amount of very
/// close extra points and it is therefore recommented to almost always call
/// [`simplify`](./fn.simplify.html) on the path after inflating/shrinking it.
///
/// `miter_limit` is scaled with the points like a distance, although
/// Clipper2 reads it as a ratio of the miter length to `delta`: with a
/// scaler other than [`One`](crate::One) the limit is in effect much larger than
/// given. [`inflate_with_arc_tolerance`](./fn.inflate_with_arc_tolerance.html)
/// passes it as the ratio it is.
///
/// # Example
///
/// ```rust
/// use clipper2::*;
///
/// let paths: Paths = vec![(2.0, 2.0), (6.0, 2.0), (6.0, 10.0), (2.0, 6.0)].into();
///
/// let output = inflate(paths, 1.0, JoinType::Round, EndType::Polygon, 0.0);
/// let output = simplify(output, 0.01, false);
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
    let paths = without_empty(paths.into());

    unsafe {
        let paths_ptr = paths.to_clipperpaths64();
        let mem = malloc(clipper_paths64_size());
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

/// Like [`inflate`](./fn.inflate.html), with control over how closely round
/// joins and round ends follow the true arc.
///
/// `arc_tolerance` is the largest distance, in the same units as the paths,
/// between a rounded corner and the true arc; it is scaled with the points.
/// With `inflate` the tolerance follows the offset instead (a fixed share of
/// `delta`), so large offsets get coarse corners. `miter_limit` is the ratio
/// of the miter length to `delta` beyond which miter joins are squared off
/// (Clipper2's default is 2.0); unlike `inflate`, it is not scaled.
///
/// # Example
///
/// ```rust
/// use clipper2::*;
///
/// let square: Paths<Milli> = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)].into();
/// let grown = inflate_with_arc_tolerance(square, 3.0, JoinType::Round, EndType::Polygon, 2.0, 0.001);
///
/// // Every point stays within 0.001 of the true rounded square.
/// let area = 100.0 + 4.0 * 10.0 * 3.0 + std::f64::consts::PI * 9.0;
/// assert!((grown.signed_area() - area).abs() < 0.05);
/// ```
pub fn inflate_with_arc_tolerance<P: PointScaler>(
    paths: impl Into<Paths<P>>,
    delta: f64,
    join_type: JoinType,
    end_type: EndType,
    miter_limit: f64,
    arc_tolerance: f64,
) -> Paths<P> {
    let delta = P::scale(delta);
    let arc_tolerance = P::scale(arc_tolerance);
    let paths = without_empty(paths.into());

    unsafe {
        let offset_mem = malloc(clipper_clipperoffset_size());
        let offset_ptr = clipper_clipperoffset(offset_mem, miter_limit, arc_tolerance, 0, 0);
        let paths_ptr = paths.to_clipperpaths64();
        clipper_clipperoffset_add_paths64(offset_ptr, paths_ptr, join_type.into(), end_type.into());
        let result_mem = malloc(clipper_paths64_size());
        let result_ptr = clipper_clipperoffset_execute(result_mem, offset_ptr, delta);
        clipper_delete_paths64(paths_ptr);
        clipper_delete_clipperoffset(offset_ptr);
        let result = Paths::from_clipperpaths64(result_ptr);
        clipper_delete_paths64(result_ptr);
        result
    }
}

/// The paths without empty ones: they add nothing to an offset, and
/// Clipper2 2.0.1 dereferences null offsetting one with open ends.
fn without_empty<P: PointScaler>(paths: Paths<P>) -> Paths<P> {
    paths.into_iter().filter(|path| !path.is_empty()).collect()
}

#[cfg(test)]
mod test {
    use crate::Centi;

    use super::*;

    #[test]
    fn empty_paths_offset_with_every_end_type() {
        use crate::Path;
        let square = Path::<Centi>::from(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        for end_type in [EndType::Polygon, EndType::Joined, EndType::Butt, EndType::Square, EndType::Round] {
            for join_type in [JoinType::Square, JoinType::Bevel, JoinType::Round, JoinType::Miter] {
                let empty = Paths::<Centi>::new(vec![Path::default()]);
                assert!(inflate(empty.clone(), 1.0, join_type, end_type, 2.0).is_empty());
                assert!(inflate_with_arc_tolerance(empty, 1.0, join_type, end_type, 2.0, 0.01).is_empty());
                let mixed = Paths::new(vec![Path::default(), square.clone()]);
                let alone = inflate(square.clone(), 1.0, join_type, end_type, 2.0);
                assert_eq!(inflate(mixed, 1.0, join_type, end_type, 2.0), alone, "{join_type:?} {end_type:?}");
            }
        }
    }

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

    #[test]
    fn arc_tolerance_bounds_rounded_corners() {
        use crate::Milli;

        let square = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        for tolerance in [0.01, 0.001] {
            let grown = inflate_with_arc_tolerance::<Milli>(
                square.clone(),
                3.0,
                JoinType::Round,
                EndType::Polygon,
                2.0,
                tolerance,
            );
            let points: Vec<Vec<(f64, f64)>> = grown.into();
            assert_eq!(points.len(), 1);
            let ring = &points[0];
            // Every point lies on the rounded square, and every chord across a
            // corner sags at most the tolerance below the arc.
            for i in 0..ring.len() {
                let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
                let mid = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
                let corner = (mid.0.clamp(0.0, 10.0), mid.1.clamp(0.0, 10.0));
                let distance = (mid.0 - corner.0).hypot(mid.1 - corner.1);
                assert!(
                    distance >= 3.0 - tolerance - 0.001,
                    "chord sags {} below the arc",
                    3.0 - distance
                );
                for p in [a, b] {
                    let corner = (p.0.clamp(0.0, 10.0), p.1.clamp(0.0, 10.0));
                    assert!(((p.0 - corner.0).hypot(p.1 - corner.1) - 3.0).abs() <= 0.001);
                }
            }
        }
    }

    #[test]
    fn inflate_keeps_its_relative_arc_tolerance() {
        let square = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        let coarse: Vec<Vec<(f64, f64)>> =
            inflate::<crate::Milli>(square.clone(), 3.0, JoinType::Round, EndType::Polygon, 2.0)
                .into();
        let fine: Vec<Vec<(f64, f64)>> = inflate_with_arc_tolerance::<crate::Milli>(
            square,
            3.0,
            JoinType::Round,
            EndType::Polygon,
            2.0,
            0.0005,
        )
        .into();
        assert!(fine[0].len() > coarse[0].len());
    }
}
