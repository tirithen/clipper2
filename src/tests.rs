use crate::*;

#[test]
fn test_union_from_vec_of_vec() {
    let shape1 = vec![(1.0, 1.0), (1.0, 5.0), (5.0, 5.0), (5.0, 1.0)];
    let shape2 = vec![(2.0, 2.0), (2.0, 4.0), (4.0, 4.0), (4.0, 2.0)];
    let input = vec![shape1.clone(), shape2];

    let paths = Paths::from(input);

    let result = union(&paths, FillRule::EvenOdd);

    assert_eq!(result.iter().count(), 1);
    assert_eq!(result.to_vec(), Paths::from(shape1).to_vec());
}