use clipper2::*;
use helpers::draw_paths;
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Intersect")]
async fn main() {
    let path_a: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    let path_b: Paths = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)].into();

    let result = intersect::<Centi>(path_a.clone(), path_b.clone(), FillRule::default())
        .expect("Failed to run boolean operation")
        .into();

    loop {
        clear_background(BLACK);
        draw_paths(&path_a, GRAY);
        draw_paths(&path_b, GRAY);
        draw_paths(&result, SKYBLUE);
        next_frame().await
    }
}
