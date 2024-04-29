use crate::{ClipType, Clipper, ClipperError, FillRule, Paths, PointScaler};

/// This function 'unions' closed subject paths, with and without clip paths.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let path_a: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
/// let path_b: Paths = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)].into();
///
/// let output: Vec<Vec<(f64, f64)>> = union(path_a, path_b, FillRule::default())
///     .expect("Failed to run boolean operation").into();
///
/// dbg!(output);
/// ```
/// ![Image displaying the result of the union example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/union.png)
///
/// For more details see [union](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Union.htm).
pub fn union<P: PointScaler>(
    subject: Paths<P>,
    clip: Paths<P>,
    fill_rule: FillRule,
) -> Result<Paths<P>, ClipperError> {
    let clipper = Clipper::<P>::new();
    clipper.add_subject(subject);
    clipper.add_clip(clip);
    clipper.boolean_operation(ClipType::Union, fill_rule)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_union() {
        let path1: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
        let path2: Paths = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)].into();
        let expected_output = vec![vec![
            (6.0, 5.0),
            (8.0, 5.0),
            (8.0, 8.0),
            (5.0, 8.0),
            (5.0, 6.0),
            (0.2, 6.0),
            (0.2, 0.2),
            (6.0, 0.2),
        ]];

        let output: Vec<Vec<(f64, f64)>> = union(path1, path2, FillRule::default()).unwrap().into();
        assert_eq!(output, expected_output);
    }
}
