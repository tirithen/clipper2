use std::marker::PhantomData;

use clipper2c_sys::{
    clipper_clipper64, clipper_clipper64_add_clip, clipper_clipper64_add_open_subject,
    clipper_clipper64_add_subject, clipper_clipper64_clear, clipper_clipper64_execute,
    clipper_clipper64_get_preserve_collinear, clipper_clipper64_get_reverse_solution,
    clipper_clipper64_set_preserve_collinear, clipper_clipper64_set_reverse_solution,
    clipper_clipper64_size, clipper_delete_clipper64, clipper_delete_paths64, ClipperClipper64,
};

use crate::{malloc, Centi, ClipType, FillRule, Paths, PointScaler};

pub(crate) struct Clipper<P: PointScaler = Centi> {
    ptr: *mut ClipperClipper64,
    _marker: PhantomData<P>,
}

impl<P: PointScaler> Clipper<P> {
    pub fn new() -> Self {
        let ptr = unsafe {
            let mem = malloc(clipper_clipper64_size());
            clipper_clipper64(mem)
        };
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn set_preserve_collinear(&self, value: bool) {
        unsafe { clipper_clipper64_set_preserve_collinear(self.ptr, if value { 1 } else { 0 }) }
    }

    pub fn get_preserve_collinear(&self) -> bool {
        unsafe { clipper_clipper64_get_preserve_collinear(self.ptr) == 1 }
    }

    pub fn set_reverse_solution(&self, value: bool) {
        unsafe { clipper_clipper64_set_reverse_solution(self.ptr, if value { 1 } else { 0 }) }
    }

    pub fn get_reverse_solution(&self) -> bool {
        unsafe { clipper_clipper64_get_reverse_solution(self.ptr) == 1 }
    }

    pub fn clear(&self) {
        unsafe { clipper_clipper64_clear(self.ptr) }
    }

    pub fn add_open_subject(&self, open_subject: Paths<P>) {
        unsafe {
            let open_subject_ptr = open_subject.to_clipperpaths64();
            clipper_clipper64_add_open_subject(self.ptr, open_subject_ptr);
            clipper_delete_paths64(open_subject_ptr);
        }
    }

    pub fn add_subject(&self, subject: Paths<P>) {
        unsafe {
            let subject_ptr = subject.to_clipperpaths64();
            clipper_clipper64_add_subject(self.ptr, subject_ptr);
            clipper_delete_paths64(subject_ptr);
        }
    }

    pub fn add_clip(&self, clip: Paths<P>) {
        unsafe {
            let clip_ptr = clip.to_clipperpaths64();
            clipper_clipper64_add_clip(self.ptr, clip_ptr);
            clipper_delete_paths64(clip_ptr);
        }
    }

    pub fn boolean_operation(
        &self,
        clip_type: ClipType,
        fill_rule: FillRule,
    ) -> Result<Paths<P>, ClipperError> {
        let closed_path = unsafe { Paths::<P>::new(Vec::new()).to_clipperpaths64() };
        let open_path = unsafe { Paths::<P>::new(Vec::new()).to_clipperpaths64() };

        unsafe {
            let success = clipper_clipper64_execute(
                self.ptr,
                clip_type.into(),
                fill_rule.into(),
                closed_path,
                open_path,
            );

            if success != 1 {
                return Err(ClipperError::FailedBooleanOperation);
            }

            let path = Paths::from_clipperpaths64(closed_path);
            clipper_delete_paths64(closed_path);
            clipper_delete_paths64(open_path);
            Ok(path)
        }
    }
}

impl Default for Clipper<Centi> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: PointScaler> Drop for Clipper<P> {
    fn drop(&mut self) {
        unsafe { clipper_delete_clipper64(self.ptr) }
    }
}

/// Errors that can occur during clipper operations.
#[derive(Debug, thiserror::Error)]
pub enum ClipperError {
    /// Failed execute boolean operation.
    #[error("Failed boolean operation")]
    FailedBooleanOperation,
}
