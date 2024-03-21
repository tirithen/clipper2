use clipper2::{Path, PathType, Polygon, Polygons, Vertex};

fn main() {
    let path1 = Path::new(
        vec![
            Vertex::new(0.1, 0.0),
            Vertex::new(10.0, 0.0),
            Vertex::new(10.0, 10.0),
            Vertex::new(0.0, 10.0),
        ],
        true,
    );
    let path2 = Path::new(
        vec![
            Vertex::new(5.0, 5.0),
            Vertex::new(15.0, 5.0),
            Vertex::new(15.0, 15.0),
            Vertex::new(5.0, 15.0),
        ],
        true,
    );
    let polygons = Polygons::new(vec![Polygon::new(
        vec![path1.clone(), path2.clone()],
        PathType::Subject,
    )]);

    let output = clipper2::union(polygons);
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
