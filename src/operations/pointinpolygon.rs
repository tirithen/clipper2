use clipper2c_sys::{clipper_delete_path64, clipper_point_in_path64};

use crate::{Path, Point, PointInPolygonResult, PointScaler};

/// The function result indicates whether the point is inside, or outside, or on
/// one of the specified polygon's edges.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let path = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
///
/// let output = point_in_polygon::<Centi>(Point::new(0.5, 0.5), &path.into());
///
/// dbg!(output);
/// ```
///
/// For more details see [point-in-polygon](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/PointInPolygon.htm).
pub fn point_in_polygon<P: PointScaler>(point: Point<P>, path: &Path<P>) -> PointInPolygonResult {
    let point_ptr = point.as_clipperpoint64();
    let path_ptr = unsafe { path.to_clipperpath64() };
    let result = unsafe { clipper_point_in_path64(path_ptr, *point_ptr) };
    unsafe { clipper_delete_path64(path_ptr) };
    result.into()
}

#[cfg(test)]
mod test {
    use crate::Centi;

    use super::*;

    #[test]
    fn test_point_in_polygon() {
        let path = vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.2, 0.2),
            (1.0, 1.0),
            (0.5, 1.0),
            (0.0, 1.0),
        ]
        .into();

        let output = point_in_polygon::<Centi>(Point::new(-10.0, 0.0), &path);
        assert_eq!(output, PointInPolygonResult::IsOutside);

        let output = point_in_polygon::<Centi>(Point::new(0.5, 0.5), &path);
        assert_eq!(output, PointInPolygonResult::IsInside);

        let output = point_in_polygon::<Centi>(Point::new(0.0, 0.0), &path);
        assert_eq!(output, PointInPolygonResult::IsOn);
    }
}
