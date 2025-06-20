pub struct ClipperOffset {
    ptr: *mut clipper2c_sys::ClipperClipperOffset,
}

impl ClipperOffset {
    pub fn new(config: ClipperOffsetConfig) -> Self {
        let ptr = unsafe {
            let mem = crate::malloc(clipper2c_sys::clipper_clipperoffset_size());
            clipper2c_sys::clipper_clipperoffset(
                mem,
                config.miter_limit,
                config.arc_tolerance,
                config.preserve_collinear,
                config.reverse_solution,
            )
        };
        Self { ptr }
    }

    pub fn add_path(&self, p: crate::Path, jt: clipper2c_sys::ClipperJoinType, et: clipper2c_sys::ClipperEndType) {
        unsafe {
            clipper2c_sys::clipper_clipperoffset_add_path64(self.ptr, p.to_clipperpath64(), jt, et);
        }
    }

    pub fn execute(&self, delta: f64) -> crate::Paths {
        unsafe {
            let mem = crate::malloc(clipper2c_sys::clipper_paths64_size());

            crate::Paths::from_clipperpaths64(clipper2c_sys::clipper_clipperoffset_execute(
                mem, self.ptr, delta,
            ))
        }
    }
}

pub struct ClipperOffsetConfig {
    /// Default: 2.0,
    miter_limit: f64,
    /// Default: 0.0,
    arc_tolerance: f64,
    /// Default: false
    preserve_collinear: ::std::os::raw::c_int,
    /// Default: false
    reverse_solution: ::std::os::raw::c_int,
}

impl Default for ClipperOffsetConfig {
    fn default() -> Self {
        Self {
            miter_limit: 2.0,
            arc_tolerance: 0.0,
            preserve_collinear: 0,
            reverse_solution: 0,
        }
    }
}

impl ClipperOffsetConfig {
    pub fn new(
        miter_limit: f64,
        arc_tolerance: f64,
        preserve_collinear: bool,
        reverse_solution: bool,
    ) -> Self {
        Self {
            miter_limit,
            arc_tolerance,
            preserve_collinear: preserve_collinear as ::std::os::raw::c_int,
            reverse_solution: reverse_solution as ::std::os::raw::c_int,
        }
    }
}
