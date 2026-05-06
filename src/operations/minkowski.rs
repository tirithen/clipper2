use clipper2c_sys::{
    clipper_delete_path64, clipper_delete_paths64, clipper_paths64_minkowski_diff,
    clipper_paths64_minkowski_sum, clipper_paths64_size,
};

use crate::{FillRule, Path, Paths, PointScaler, malloc};

/// Sweep `pattern` along `paths`, returning the union of all
/// translated copies of `pattern` placed at every vertex of every
/// input path.
///
/// Geometrically this is `{ a + b | a ∈ pattern, b ∈ path }` for
/// each input path, with the per-path results unioned together.
/// It is the right primitive when you need to grow a shape by an
/// arbitrary polygonal kernel rather than the radius-only kernel
/// that [`inflate`](./fn.inflate.html) offers — for example a square
/// cutter, a drag-knife, or any other tool footprint that is not
/// disc-shaped. The kernel may be concave; a concave kernel can
/// carve holes into the swept region, and the returned [`Paths`]
/// then contains both the outer ring and the carved inner rings.
///
/// Set `is_closed` to `true` when the input paths are closed
/// polygons — the usual case for boundary growth, configuration-space
/// obstacles, and tool-clearance regions. Set it to `false` when they
/// are open polylines, e.g. drag-knife / profile-cutter centerlines
/// or stroke generation.
///
/// The internal per-path union uses [`FillRule::NonZero`]; the
/// pattern is applied at every vertex of every input path, so a
/// dense polyline will produce a denser result than a polygon with
/// the same outer shape.
///
/// See [`minkowski_diff`] for the companion that translates by `-p`
/// instead of `+p` at every vertex; for an asymmetric kernel
/// difference produces the sum reflected through the kernel's
/// origin.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// // A concave arrowhead pointing right (-0.5..0.6 in x, ±0.5 in y).
/// let pattern: Path = vec![
///     (0.6, 0.0),
///     (-0.5, 0.5),
///     (-0.1, 0.0),
///     (-0.5, -0.5),
/// ].into();
///
/// // A closed L-shaped contour — the kind of outline you would
/// // boundary-grow in a CAD/CAM toolpath.
/// let outline: Path = vec![
///     (1.0, 1.0), (5.0, 1.0), (5.0, 2.0),
///     (2.5, 2.0), (2.5, 3.5), (1.0, 3.5),
/// ].into();
///
/// let swept = minkowski_sum(pattern, outline, true);
/// dbg!(swept);
/// ```
///
/// In the screenshot below, the gray outline is the input L-shape
/// and the sky-blue ring is the Minkowski sum of that L with the
/// arrowhead pattern — note the asymmetric horizontal extension
/// (the arrowhead is wider in x than in y) and the small inner ring
/// carved by the arrow's concave notch.
///
/// ![Image displaying the result of the Minkowski sum example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/minkowski-sum.png)
///
/// For more details see the original [Minkowski sum](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/MinkowskiSum.htm) docs.
pub fn minkowski_sum<P: PointScaler>(
    pattern: impl Into<Path<P>>,
    paths: impl Into<Paths<P>>,
    is_closed: bool,
) -> Paths<P> {
    minkowski(pattern, paths, is_closed, MinkowskiOp::Sum)
}

/// Sweep `pattern` along `paths` translating by `-p` instead of
/// `+p` at each vertex.
///
/// For a pattern that is symmetric about the origin (a centred disc
/// or square) sum and difference produce the same result; the
/// distinction matters as soon as the pattern is asymmetric, where
/// difference is the operation behind "set of points `x` such that
/// `x + pattern` is contained in `path`" intuitions — robot
/// footprint configuration spaces, tool-reachability inside a
/// pocket, swept volumes for a cutter approaching from a fixed
/// direction.
///
/// See [`minkowski_sum`] for the meaning of `is_closed` and the
/// internal fill rule. For an asymmetric pattern, the difference
/// looks like the sum reflected through the origin of the pattern,
/// which is exactly what you can see by comparing the two
/// screenshots: the same arrowhead pattern extends the swept region
/// rightward under [`minkowski_sum`] and leftward under
/// `minkowski_diff`.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// // Same concave arrowhead as the minkowski_sum example.
/// let pattern: Path = vec![
///     (0.6, 0.0),
///     (-0.5, 0.5),
///     (-0.1, 0.0),
///     (-0.5, -0.5),
/// ].into();
///
/// // Same L-shape outline as the minkowski_sum example.
/// let outline: Path = vec![
///     (1.0, 1.0), (5.0, 1.0), (5.0, 2.0),
///     (2.5, 2.0), (2.5, 3.5), (1.0, 3.5),
/// ].into();
///
/// let swept = minkowski_diff(pattern, outline, true);
/// dbg!(swept);
/// ```
///
/// In the screenshot below, the gray outline is the same input
/// L-shape used in the [`minkowski_sum`] example and the sky-blue
/// rings are the Minkowski difference of that L with the same
/// arrowhead pattern — the swept boundary now extends to the **left**
/// of the L (mirror of the sum result, where it extended right) and
/// the inner ring carved by the concave notch sits in the **left**
/// half of the L instead of the right.
///
/// ![Image displaying the result of the Minkowski difference example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/minkowski-diff.png)
///
/// For more details see the original [Minkowski difference](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/MinkowskiDiff.htm) docs.
pub fn minkowski_diff<P: PointScaler>(
    pattern: impl Into<Path<P>>,
    paths: impl Into<Paths<P>>,
    is_closed: bool,
) -> Paths<P> {
    minkowski(pattern, paths, is_closed, MinkowskiOp::Diff)
}

#[derive(Clone, Copy)]
enum MinkowskiOp {
    Sum,
    Diff,
}

fn minkowski<P: PointScaler>(
    pattern: impl Into<Path<P>>,
    paths: impl Into<Paths<P>>,
    is_closed: bool,
    op: MinkowskiOp,
) -> Paths<P> {
    let pattern: Path<P> = pattern.into();
    let paths: Paths<P> = paths.into();

    unsafe {
        let mem = malloc(clipper_paths64_size());
        let pattern_ptr = pattern.to_clipperpath64();
        let paths_ptr = paths.to_clipperpaths64();
        let result_ptr = match op {
            MinkowskiOp::Sum => clipper_paths64_minkowski_sum(
                mem,
                pattern_ptr,
                paths_ptr,
                is_closed.into(),
                FillRule::NonZero.into(),
            ),
            MinkowskiOp::Diff => clipper_paths64_minkowski_diff(
                mem,
                pattern_ptr,
                paths_ptr,
                is_closed.into(),
                FillRule::NonZero.into(),
            ),
        };
        clipper_delete_path64(pattern_ptr);
        clipper_delete_paths64(paths_ptr);
        let result = Paths::from_clipperpaths64(result_ptr);
        clipper_delete_paths64(result_ptr);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Centi;

    #[test]
    fn minkowski_sum_unit_square_along_segment() {
        let pattern: Path<Centi> = vec![(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)].into();
        let path: Path<Centi> = vec![(0.0, 0.0), (4.0, 0.0)].into();

        let result = minkowski_sum(pattern, path, false);
        assert!(!result.is_empty());

        let bounds = result.bounds();
        assert!((bounds.min.x() - -0.5).abs() < 1e-9);
        assert!((bounds.max.x() - 4.5).abs() < 1e-9);
        assert!((bounds.min.y() - -0.5).abs() < 1e-9);
        assert!((bounds.max.y() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn minkowski_sum_and_diff_agree_for_symmetric_pattern() {
        let pattern: Path<Centi> = vec![(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)].into();
        let path: Path<Centi> = vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)].into();

        let sum = minkowski_sum(pattern.clone(), path.clone(), true);
        let diff = minkowski_diff(pattern, path, true);

        assert_eq!(sum.bounds().min, diff.bounds().min);
        assert_eq!(sum.bounds().max, diff.bounds().max);
    }

    #[test]
    fn minkowski_sum_covers_all_input_paths() {
        let pattern: Path<Centi> = vec![(-0.2, -0.2), (0.2, -0.2), (0.2, 0.2), (-0.2, 0.2)].into();
        let paths: Paths<Centi> = vec![
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            vec![(3.0, 0.0), (4.0, 0.0), (4.0, 1.0), (3.0, 1.0)],
        ]
        .into();

        let result = minkowski_sum(pattern, paths, true);

        assert!(!result.is_empty());
        let bounds = result.bounds();
        assert!((bounds.min.x() - -0.2).abs() < 1e-9);
        assert!((bounds.max.x() - 4.2).abs() < 1e-9);
        assert!((bounds.min.y() - -0.2).abs() < 1e-9);
        assert!((bounds.max.y() - 1.2).abs() < 1e-9);
    }
}
