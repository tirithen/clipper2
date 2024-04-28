use std::slice;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Paths {
    points: *const Point,
    path_starts: *const usize,
    path_capacities: *const usize,
    num_paths: usize,
    num_paths_capacity: usize,
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
            let path_capacities = get_path_capacities(ptr);
            let num_paths = get_num_paths(ptr);
            let num_paths_capacity = get_num_paths_capacity(ptr);

            free_paths_c(ptr);

            Paths {
                points,
                path_starts,
                path_capacities,
                num_paths,
                num_paths_capacity,
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

impl From<Paths> for Vec<Vec<(f64, f64)>> {
    fn from(vec: Paths) -> Self {
        vec.iter()
            .map(|path| path.iter().map(|point| (point.x(), point.y())).collect())
            .collect()
    }
}

impl From<Paths> for Vec<Vec<[f64; 2]>> {
    fn from(vec: Paths) -> Self {
        vec.iter()
            .map(|path| path.iter().map(|point| [point.x(), point.y()]).collect())
            .collect()
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
            .map(|inner_vec| inner_vec.into_iter().map(Point::from).collect())
            .collect();
        Paths::from_vec(vec_of_vec_points)
    }
}

impl From<Vec<Vec<[f64; 2]>>> for Paths {
    fn from(vec: Vec<Vec<[f64; 2]>>) -> Self {
        let paths: Vec<Vec<Point>> = vec
            .into_iter()
            .map(|path| path.into_iter().map(Point::from).collect())
            .collect();
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
        let points: Vec<Point> = vec.into_iter().map(Point::from).collect();
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
            let _ = Vec::from_raw_parts(
                self.path_starts as *mut usize,
                path_starts_len,
                path_starts_capacity,
            );
        }
    }
}
