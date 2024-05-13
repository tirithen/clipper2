use clipper2::*;
use helpers::{circle_path, draw_paths};
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Difference and inflate")]
async fn main() -> Result<(), ClipperError> {
    let circle = circle_path((5.0, 5.0), 3.0, 32);
    let circle2 = circle_path((6.0, 6.0), 2.0, 32);
    let circle3 = circle_path((7.0, 7.0), 1.0, 32);
    let rectangle = vec![(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (0.0, 6.0)];

    // Functional API
    let _result = difference(circle.clone(), circle2.clone(), FillRule::default())?;
    let _result = difference(_result, circle3.clone(), FillRule::default())?;
    let _result = difference(_result, rectangle.clone(), FillRule::default())?;

    let _result2 = inflate(_result, 1.0, JoinType::Round, EndType::Polygon, 0.0);
    let _result2 = simplify(_result2, 0.01, false);

    // Alternative Clipper builder API
    let result = circle
        .to_clipper_subject()
        .add_clip(circle2)
        .add_clip(circle3)
        .add_clip(rectangle)
        .difference(FillRule::default())?;

    let result2 = result
        .inflate(1.0, JoinType::Round, EndType::Polygon, 0.0)
        .simplify(0.01, false);

    loop {
        clear_background(BLACK);
        draw_paths(&result, SKYBLUE);
        draw_paths(&result2, GREEN);
        next_frame().await
    }
}
