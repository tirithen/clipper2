use clipper2::*;
use helpers::{circle_path, draw_paths};
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Difference and inflate")]
async fn main() {
    let circle = circle_path((5.0, 5.0), 3.0, 32);
    let rectangle: Paths = vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)].into();
    let circle2 = circle_path((7.0, 7.0), 1.0, 32);

    let result = difference(circle, rectangle, FillRule::default())
        .expect("Failed to run boolean operation");

    let result = difference(result.clone(), circle2, FillRule::default())
        .expect("Failed to run boolean operation");

    let result2 = inflate(result.clone(), 1.0, JoinType::Round, EndType::Polygon, 0.0);

    loop {
        clear_background(BLACK);
        draw_paths(&result, SKYBLUE);
        draw_paths(&result2, GREEN);
        next_frame().await
    }
}
