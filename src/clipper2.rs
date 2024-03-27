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
    pub x: i64,
    pub y: i64,
}

pub fn union(subjects: Paths, fill_rule: FillRule) -> Paths {
    let ffi_subjects = convert_paths_to_ffi(&subjects);
    let ffi_fill_rule = fill_rule.into(); // Assuming conversion to C enum.

    unsafe {
        let result_ffi = union_c(ffi_subjects, ffi_fill_rule);
        let result = convert_ffi_to_paths(result_ffi);
        // Ensure to free the C++ allocated `RustFriendlyPathsC` object after use.
        free_rust_friendly_paths_c(result_ffi);
        result
    }
}

fn convert_paths_to_ffi(paths: &Paths) -> *mut RustFriendlyPathsC {
    let num_paths = paths.len();
    let paths_ptrs: Vec<*const Point> = paths.iter().map(|path| path.as_ptr()).collect();
    let lengths: Vec<usize> = paths.iter().map(|path| path.len()).collect();

    // Convert Vecs to raw pointers for FFI.
    // Note: It's crucial to ensure these don't get dropped while in use by C++.
    let paths_ptr = paths_ptrs.as_ptr();
    let lengths_ptr = lengths.as_ptr();

    unsafe {
        // Call the C++ function that creates RustFriendlyPathsC from provided parts.
        create_rust_friendly_paths_c(num_paths, paths_ptr, lengths_ptr)
    }
}

unsafe fn convert_ffi_to_paths(rust_paths_c: *const RustFriendlyPathsC) -> Paths {
    let num_paths = get_num_paths(rust_paths_c);
    let paths_ptr = get_paths_ptr(rust_paths_c);
    let path_lengths_ptr = get_path_lengths_ptr(rust_paths_c);

    (0..num_paths)
        .map(|i| {
            let path_ptr = *paths_ptr.add(i);
            let path_len = *path_lengths_ptr.add(i);
            let path_slice = std::slice::from_raw_parts(path_ptr, path_len);
            path_slice
                .iter()
                .map(|&point| point.into())
                .collect::<Vec<Point>>()
        })
        .collect()
}
