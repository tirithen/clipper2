#![allow(dead_code)]

use std::f64::consts::PI;

use clipper2::*;
use macroquad::prelude::*;

const SCALE: f32 = 50.0;

pub fn draw_paths(paths: &Paths, color: Color) {
    for path in paths.iter() {
        draw_path(path, color);
    }
}

pub fn draw_path(path: &Path, color: Color) {
    let mut last_point = path.iter().last().unwrap_or(&Point::ZERO);

    for point in path.iter() {
        draw_line(
            last_point.x() as f32 * SCALE,
            last_point.y() as f32 * SCALE,
            point.x() as f32 * SCALE,
            point.y() as f32 * SCALE,
            3.0,
            color,
        );
        last_point = point;
    }
}

pub fn circle_path(offset: (f64, f64), radius: f64, segments: usize) -> Paths {
    let mut points = vec![];

    for i in 0..segments {
        let angle = (i as f64 / segments as f64) * 2.0 * PI;
        points.push((
            angle.sin() * radius + offset.0,
            angle.cos() * radius + offset.1,
        ));
    }

    points.into()
}

// Dummy main to pass tests as module is for exporting helpers

fn main() {}
