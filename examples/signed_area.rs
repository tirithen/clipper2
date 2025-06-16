use clipper2::*;

fn main() {
    let path_a: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    let area = path_a.signed_area();
    println!("Area of the path: {}", area);
}
