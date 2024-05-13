use clipper2::*;
use helpers::{draw_path, draw_path_points};
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Simplify")]
async fn main() {
    let path: Path = vec![(1.0, 2.0), (1.0, 2.5), (1.2, 4.0), (1.8, 6.0)].into();

    // Functional API
    let _path = path.translate(3.0, 3.0);
    let _simplified = simplify(path.clone(), 0.5, false);

    // Alternative paths API
    let simplified = path.translate(3.0, 0.0).simplify(0.5, false);

    loop {
        clear_background(BLACK);
        draw_path(&path, SKYBLUE);
        draw_path(&simplified, SKYBLUE);
        draw_path_points(&path, GREEN);
        draw_path_points(&simplified, GREEN);
        next_frame().await
    }
}
