use crate::*;

#[test]
fn test_paths_from_into() {
    let shape1 = vec![(1.0, 1.0), (1.0, 5.0), (5.0, 5.0), (5.0, 1.0)];
    let shape2 = vec![(2.0, 2.0), (2.0, 4.0), (8.0, 4.0), (8.0, 2.0)];
    let shapes = vec![shape1.clone(), shape2];

    let paths = Paths::from(shapes.clone());
    let paths_vec: Vec<Vec<(f64, f64)>> = paths.into();

    assert_eq!(paths_vec, shapes);
}

#[test]
fn test_union() {
    let shape1 = vec![(1.0, 1.0), (1.0, 5.0), (5.0, 5.0), (5.0, 1.0)];
    let shape2 = vec![(2.0, 2.0), (2.0, 4.0), (8.0, 4.0), (8.0, 2.0)];
    // let shape3 = vec![(20.0, 20.0), (20.0, 40.0), (80.0, 40.0), (80.0, 20.0)];
    let shapes = vec![shape1, shape2, /*shape3*/];

    let paths = Paths::from(shapes);
    let result = union(&paths, FillRule::NonZero);

    assert_eq!(result.iter().count(), 2);
    assert_eq!(result.to_vec(), Paths::from(vec![vec![(1.0, 1.0), (1.0, 5.0), (5.0, 5.0), (5.0, 1.0)]]).to_vec());
}