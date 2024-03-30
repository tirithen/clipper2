use clipper2::{union, FillRule, Paths};

fn main() {
    let path1 = vec![
        [1.0, 0.0],
        [10.0, 0.0],
        [10.0, 10.0],
        [0.0, 10.0],
    ];
    let path2 = vec![
        [5.0, 5.0],
        [15.0, 5.0],
        [15.0, 15.0],
        [5.0, 15.0],
    ];

    let output = union(&Paths::from(vec![path1, path2]), FillRule::NonZero);

    println!("Points: {:?}", output.to_vec());
}
