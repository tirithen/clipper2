use clipper2::*;
use helpers::draw_paths;
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Inflate")]
async fn main() {
    let path: Paths = vec![(2.0, 2.0), (6.0, 2.0), (6.0, 10.0), (2.0, 6.0)].into();

    // Functional API
    let _result = inflate(path.clone(), 1.0, JoinType::Round, EndType::Polygon, 0.0);

    // Alternative paths API
    let result = path.inflate(1.0, JoinType::Round, EndType::Polygon, 0.0);

    loop {
        clear_background(BLACK);
        draw_paths(&path, GRAY);
        draw_paths(&result, SKYBLUE);
        next_frame().await
    }
}
