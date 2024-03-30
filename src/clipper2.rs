#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[cfg(all(not(feature = "update-bindings"), feature = "generate-bindings"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(any(feature = "update-bindings", not(feature = "generate-bindings")))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/bindings.rs"
));

const PRECISION_MULTIPLIER: f64 = 100.0;

#[derive(Debug, Copy, Clone)]
pub enum FillRule {
    EvenOdd,
    NonZero,
    Positive,
    Negative,
}

impl From<FillRule> for FillRuleC {
    fn from(fill_rule: FillRule) -> Self {
        match fill_rule {
            FillRule::EvenOdd => FillRuleC_EvenOdd,
            FillRule::NonZero => FillRuleC_NonZero,
            FillRule::Positive => FillRuleC_Positive,
            FillRule::Negative => FillRuleC_Negative,
        }
    }
}

pub type Paths = Vec<Path>;
pub type Path = Vec<Point>;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: (x * PRECISION_MULTIPLIER) as i64,
            y: (y * PRECISION_MULTIPLIER) as i64,
        }
    }

    pub fn from_scaled(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x as f64 / PRECISION_MULTIPLIER
    }

    pub fn y(&self) -> f64 {
        self.y as f64 / PRECISION_MULTIPLIER
    }

    pub fn x_scaled(&self) -> i64 {
        self.x
    }

    pub fn y_scaled(&self) -> i64 {
        self.y
    }
}

impl From<Point> for PointC {
    fn from(point: Point) -> Self {
        PointC { x: point.x, y: point.y }
    }
}

impl From<PointC> for Point {
    fn from(point: PointC) -> Self {
        Point { x: point.x, y: point.y }
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Point::new(x, y)
    }
}

fn union(paths: &[Path], fill_rule: FillRuleC) -> Vec<Path> {
    // Convert Rust Paths to C-compatible format.
    let (flat_points, path_sizes) = convert_paths_to_c_format(paths);

    // Call the C function.
    let raw_result = unsafe {
        union_c(
            flat_points.as_ptr(),
            paths.len(),
            path_sizes.as_ptr(),
            fill_rule,
        )
    };

    // Convert the result back to Rust format.
    let result = unsafe { convert_paths_c_to_rust(raw_result) };

    // Free the C-allocated memory.
    unsafe {
        free_paths_c(raw_result);
    }

    result
}

// Convert a slice of Paths to a flat vector of Points and a vector of path sizes.
fn convert_paths_to_c_format(paths: &[Path]) -> (Vec<PointC>, Vec<usize>) {
    let flat_points: Vec<PointC> = paths.iter().flat_map(|path| path.iter().cloned()).collect();
    let path_sizes: Vec<usize> = paths.iter().map(|path| path.len()).collect();

    (flat_points, path_sizes)
}

// Convert the C result back to a Vec<Path>.
unsafe fn convert_paths_c_to_rust(paths_c: *mut PathsC) -> Vec<Path> {
    // Assuming we have access to PathsC fields or equivalent getter functions.
    let paths = std::slice::from_raw_parts((*paths_c).points, (*paths_c).num_paths);
    let path_starts = std::slice::from_raw_parts((*paths_c).path_starts, (*paths_c).num_paths + 1);

    let mut result = Vec::new();

    for i in 0..(*paths_c).num_paths {
        let start = path_starts[i] as usize;
        let end = path_starts[i + 1] as usize; // Assuming there's an extra entry indicating the total end.
        result.push(paths[start..end].to_vec());
    }

    result
}
