use clipper2::*;
use helpers::draw_paths;
use macroquad::prelude::*;

mod helpers;

#[macroquad::main("Minkowski sum")]
async fn main() {
    // A concave arrowhead pointing right — wider in x than in y, with
    // a notch carved out of its back. This is the kind of footprint
    // where a circular `inflate` produces the wrong shape: the kernel
    // is neither convex nor disc-shaped, so the swept band is wider
    // along horizontal edges than vertical ones, the outer corners
    // pick up an arrow-shaped bulge, and the concave notch carves a
    // hole into the swept region.
    let pattern: Path = vec![
        (0.6, 0.0),   // arrow tip
        (-0.5, 0.5),  // upper-left tail
        (-0.1, 0.0),  // concave notch (the back of the arrow)
        (-0.5, -0.5), // lower-left tail
    ]
    .into();

    // A closed L-shaped polygon — the kind of contour you'd
    // boundary-grow in a CAD/CAM toolpath. Sized 4x2.5 world units so
    // the bounding box is landscape-oriented (matching macroquad's
    // default window aspect) with comfortable margin around the
    // arrow-shaped extensions added by the sweep.
    let outline: Path = vec![
        (1.0, 1.0),
        (5.0, 1.0),
        (5.0, 2.0),
        (2.5, 2.0),
        (2.5, 3.5),
        (1.0, 3.5),
    ]
    .into();

    let swept = outline.minkowski_sum(pattern, true);
    let original: Paths = vec![outline].into();

    loop {
        clear_background(BLACK);
        draw_paths(&original, GRAY);
        draw_paths(&swept, SKYBLUE);
        next_frame().await
    }
}
