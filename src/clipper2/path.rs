use std::slice;

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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone)]
pub struct Paths {
    points: *const Point,
    path_starts: *const usize,
    num_paths: usize,
}

impl Paths {
    pub fn from_vec(vec: Vec<Vec<Point>>) -> Self {
        let mut flat_points = vec![];
        let mut path_starts = vec![];
        let mut current_start = 0;

        for path in vec {
            path_starts.push(current_start);
            current_start += path.len();
            flat_points.extend(path);
        }
        path_starts.push(current_start);

        let points_ptr = flat_points.as_ptr() as *mut Point;
        let path_starts_ptr = path_starts.as_ptr() as *mut usize;

        let paths = Paths {
            points: points_ptr,
            path_starts: path_starts_ptr,
            num_paths: path_starts.len() - 1,
        };

        std::mem::forget(flat_points);
        std::mem::forget(path_starts);

        paths
    }

    pub fn to_vec(&self) -> Vec<Vec<Point>> {
        self.iter().map(|path| path.to_vec()).collect()
    }

    pub(crate) fn from_raw_parts(ptr: *mut PathsC) -> Self {
        unsafe {
            let points = get_points(ptr);
            let path_starts = get_path_starts(ptr);
            let num_paths = get_num_paths(ptr);

            free_paths_c(ptr);

            Paths {
                points,
                path_starts,
                num_paths,
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &[Point]> + '_ {
        self.path_starts().windows(2).map(move |window| {
            let start = window[0];
            let end = window[1];
            unsafe { slice::from_raw_parts(self.points.add(start), end - start) }
        })
    }

    fn path_starts(&self) -> &[usize] {
        unsafe { slice::from_raw_parts(self.path_starts, self.num_paths + 1) }
    }
}

impl From<Paths> for Vec<Vec<Point>> {
    fn from(vec: Paths) -> Self {
        vec.to_vec()
    }
}

impl From<Vec<Vec<Point>>> for Paths {
    fn from(vec: Vec<Vec<Point>>) -> Self {
        Paths::from_vec(vec)
    }
}

impl From<Vec<Vec<(f64, f64)>>> for Paths {
    fn from(vec: Vec<Vec<(f64, f64)>>) -> Self {
        let vec_of_vec_points: Vec<Vec<Point>> = vec
            .into_iter()
            .map(|inner_vec|
                inner_vec.into_iter()
                    .map(Point::from)
                    .collect()
            )
            .collect();
        Paths::from_vec(vec_of_vec_points)
    }
}

impl From<Vec<Vec<[f64; 2]>>> for Paths {
    fn from(vec: Vec<Vec<[f64; 2]>>) -> Self {
        let paths: Vec<Vec<Point>> = vec.into_iter().map(|path|
            path.into_iter().map(|[x, y]| Point::from((x, y))).collect()
        ).collect();
        Paths::from_vec(paths)
    }
}

impl From<Vec<Point>> for Paths {
    fn from(vec: Vec<Point>) -> Self {
        Paths::from_vec(vec![vec])
    }
}

impl From<Vec<(f64, f64)>> for Paths {
    fn from(vec: Vec<(f64, f64)>) -> Self {
        let points: Vec<Point> = vec.into_iter().map(Point::from).collect();
        Paths::from_vec(vec![points])
    }
}

impl From<Vec<[f64; 2]>> for Paths {
    fn from(vec: Vec<[f64; 2]>) -> Self {
        let points: Vec<Point> = vec.into_iter().map(|[x, y]| Point::from((x, y))).collect();
        Paths::from_vec(vec![points])
    }
}

impl Drop for Paths {
    fn drop(&mut self) {
        unsafe {
            let points_len = self.num_paths;
            let points_capacity = self.num_paths;
            let path_starts_len = self.num_paths + 1;
            let path_starts_capacity = self.num_paths + 1;

            let _ = Vec::from_raw_parts(self.points as *mut Point, points_len, points_capacity);
            let _ = Vec::from_raw_parts(self.path_starts as *mut usize, path_starts_len, path_starts_capacity);
        }
    }
}
