use clipper2::*;
use helpers::draw_paths;
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Minkowski difference")]
async fn main() {
    // The same concave arrowhead used in the minkowski_sum example.
    // Because difference translates the pattern by `-p` instead of
    // `+p` at every vertex, an asymmetric kernel produces a result
    // that is the sum reflected through the kernel's origin: the
    // arrow-shaped protrusions extend leftward (and slightly down)
    // here where the sum example extended them rightward.
    let pattern: Path = vec![
        (0.6, 0.0),   // arrow tip
        (-0.5, 0.5),  // upper-left tail
        (-0.1, 0.0),  // concave notch (the back of the arrow)
        (-0.5, -0.5), // lower-left tail
    ]
    .into();

    // The same L-shape outline as the minkowski_sum example. For an
    // asymmetric kernel with x-extent [-0.5, +0.6] the difference
    // sweeps with the reflected kernel [-0.6, +0.5], which has the
    // same total width — so the L does not need to be shifted to
    // fit; the bias just points the other way.
    let outline: Path = vec![
        (1.0, 1.0),
        (5.0, 1.0),
        (5.0, 2.0),
        (2.5, 2.0),
        (2.5, 3.5),
        (1.0, 3.5),
    ]
    .into();

    let swept = outline.minkowski_diff(pattern, true);
    let original: Paths = vec![outline].into();

    loop {
        clear_background(BLACK);
        draw_paths(&original, GRAY);
        draw_paths(&swept, SKYBLUE);
        next_frame().await
    }
}
