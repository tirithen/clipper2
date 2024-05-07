use clipper2::*;
use helpers::{draw_paths, draw_paths_points};
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Simplify")]
async fn main() {
    let path: Paths = vec![(1.0, 2.0), (1.0, 2.5), (1.2, 4.0), (1.8, 6.0)].into();
    let path_simplified = simplify(path.translate(3.0, 0.0), 0.5, false);

    loop {
        clear_background(BLACK);
        draw_paths(&path, SKYBLUE);
        draw_paths(&path_simplified, SKYBLUE);
        draw_paths_points(&path, GREEN);
        draw_paths_points(&path_simplified, GREEN);
        next_frame().await
    }
}
