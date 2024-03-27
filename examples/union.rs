use clipper2::{Path, PathType, Polygon, Polygons, Vertex};

fn main() {
    let path1 = vec![
        [100, 0],
        [1000, 0],
        [1000, 1000],
        [0, 1000],
    ];
    let path2 = vec![
        [500, 500],
        [1500, 500],
        [1500, 1500],
        [500, 1500],
    ];

    let output = clipper2::union(vec![path1, path2], FillRule::NonZero);
    println!(
        "Vertices: {}",
        output
            .polygons()
            .first()
            .unwrap()
            .paths()
            .first()
            .unwrap()
            .vertices()
            .iter()
            .map(|v| format!("({:.1}, {:.1})", v.x(), v.y()))
            .collect::<Vec<_>>()
            .join(", ")
    );
}
