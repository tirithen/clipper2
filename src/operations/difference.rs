use crate::{ClipType, Clipper, ClipperError, FillRule, Paths, PointScaler};

/// This function differences closed subject paths from clip paths.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let path_a: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
/// let path_b: Paths = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)].into();
///
/// let output: Vec<Vec<(f64, f64)>> = difference(path_a, path_b, FillRule::default())
///     .expect("Failed to run boolean operation").into();
///
/// dbg!(output);
/// ```
///
/// ![Image displaying the result of the difference example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/difference.png)
///
/// For more details see [difference](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Difference.htm).
pub fn difference<P: PointScaler>(
    subject: Paths<P>,
    clip: Paths<P>,
    fill_rule: FillRule,
) -> Result<Paths<P>, ClipperError> {
    let clipper = Clipper::<P>::new();
    clipper.add_subject(subject);
    clipper.add_clip(clip);
    clipper.boolean_operation(ClipType::Difference, fill_rule)
}

#[cfg(test)]
mod test {
    use crate::Centi;

    use super::*;

    #[test]
    fn test_difference() {
        let path1 = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let path2 = vec![(0.5, 0.5), (1.5, 0.5), (1.5, 1.5), (0.5, 1.5)];
        let expected_output = vec![vec![
            (1.0, 0.5),
            (0.5, 0.5),
            (0.5, 1.0),
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
        ]];

        let output: Vec<Vec<(f64, f64)>> =
            difference::<Centi>(path1.into(), path2.into(), FillRule::default())
                .unwrap()
                .into();
        assert_eq!(output, expected_output);
    }
}
