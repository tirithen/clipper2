const PRECISION_MULTIPLIER: f64 = 100.0;

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

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Point::new(x, y)
    }
}

impl From<[f64; 2]> for Point {
    fn from([x, y]: [f64; 2]) -> Self {
        Point::new(x, y)
    }
}

impl From<Point> for (f64, f64) {
    fn from(point: Point) -> Self {
        (point.x(), point.y())
    }
}

impl From<Point> for [f64; 2] {
    fn from(point: Point) -> Self {
        [point.x(), point.y()]
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
