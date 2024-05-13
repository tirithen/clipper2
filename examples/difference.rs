use clipper2::*;
use helpers::draw_paths;
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Difference")]
async fn main() -> Result<(), ClipperError> {
    let path_a: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    let path_b: Paths = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)].into();

    // Functional API
    let _result = difference(path_a.clone(), path_b.clone(), FillRule::default())?;

    // Alternative Clipper builder API
    let result = path_a
        .to_clipper_subject()
        .add_clip(path_b.clone())
        .difference(FillRule::default())?;

    loop {
        clear_background(BLACK);
        draw_paths(&path_a, GRAY);
        draw_paths(&path_b, GRAY);
        draw_paths(&result, SKYBLUE);
        next_frame().await
    }
}
