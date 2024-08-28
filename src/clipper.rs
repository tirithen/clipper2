use std::marker::PhantomData;

use clipper2c_sys::{
    clipper_clipper64, clipper_clipper64_add_clip, clipper_clipper64_add_open_subject,
    clipper_clipper64_add_subject, clipper_clipper64_execute, clipper_clipper64_size,
    clipper_delete_clipper64, clipper_delete_paths64, ClipperClipper64,
};

use crate::{malloc, Centi, ClipType, FillRule, Paths, PointScaler};

/// The state of the Clipper struct.
pub trait ClipperState {}

/// A state indicating no subjects and no clips.
#[derive(Debug)]
pub struct NoSubjects {}
impl ClipperState for NoSubjects {}

/// A state indicating one or more subjects and no clips.
#[derive(Debug)]
pub struct WithSubjects {}
impl ClipperState for WithSubjects {}

/// A state indicating one or more subjects and one or more clips.
#[derive(Debug)]
pub struct WithClips {}
impl ClipperState for WithClips {}

/// The Clipper struct used as a builder for applying boolean operations to paths.
#[derive(Debug)]
pub struct Clipper<S: ClipperState = NoSubjects, P: PointScaler = Centi> {
    ptr: *mut ClipperClipper64,
    keep_ptr_on_drop: bool,
    _marker: PhantomData<P>,
    _state: S,

    /// We only hold on to this in order to avoid leaking memory when Clipper is dropped
    #[cfg(feature = "usingz")]
    raw_z_callback: Option<*mut libc::c_void>,
}

impl<P: PointScaler> Clipper<NoSubjects, P> {
    /// Creates a new empty Clipper instance.
    pub fn new() -> Clipper<NoSubjects, P> {
        let ptr = unsafe {
            let mem = malloc(clipper_clipper64_size());
            clipper_clipper64(mem)
        };

        Clipper::<NoSubjects, P> {
            ptr,
            keep_ptr_on_drop: false,
            _marker: PhantomData,
            _state: NoSubjects {},

            #[cfg(feature = "usingz")]
            raw_z_callback: None,
        }
    }
}

impl<P: PointScaler> Clipper<NoSubjects, P> {
    /// Adds a subject path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path);
    /// ```
    pub fn add_subject(mut self, subject: impl Into<Paths<P>>) -> Clipper<WithSubjects, P> {
        self.keep_ptr_on_drop = true;

        let clipper = Clipper::<WithSubjects, P> {
            ptr: self.ptr,
            keep_ptr_on_drop: false,
            _marker: PhantomData,
            _state: WithSubjects {},

            #[cfg(feature = "usingz")]
            raw_z_callback: None,
        };

        drop(self);

        clipper.add_subject(subject)
    }

    /// Adds an open subject path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    ///
    /// let clipper = Clipper::new().add_open_subject(path);
    /// ```
    pub fn add_open_subject(mut self, subject: impl Into<Paths<P>>) -> Clipper<WithSubjects, P> {
        self.keep_ptr_on_drop = true;

        let clipper = Clipper::<WithSubjects, P> {
            ptr: self.ptr,
            keep_ptr_on_drop: false,
            _marker: PhantomData,
            _state: WithSubjects {},

            #[cfg(feature = "usingz")]
            raw_z_callback: None,
        };

        drop(self);

        clipper.add_open_subject(subject)
    }

    #[cfg(feature = "usingz")]
    /// Allows specifying a callback that will be called each time a new vertex
    /// is created by Clipper, in order to set user data on such points. New
    /// points are created at the intersections between two edges, and the
    /// callback will be called with the four neighboring points from those two
    /// edges. The last argument is the new point itself.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let mut clipper = Clipper::<NoSubjects, Centi>::new();
    /// clipper.set_z_callback(|_: Point<_>, _: Point<_>, _: Point<_>, _: Point<_>, p: &mut Point<_>| {
    ///     p.set_z(1);
    /// });
    /// ```
    pub fn set_z_callback(
        &mut self,
        callback: impl Fn(
            crate::Point<P>,
            crate::Point<P>,
            crate::Point<P>,
            crate::Point<P>,
            &mut crate::Point<P>,
        ),
    ) {
        use crate::Point;

        // The closure will be represented by a trait object behind a fat
        // pointer. Since fat pointers are larger than thin pointers, they
        // cannot be passed through a thin-pointer c_void type. We must
        // therefore wrap the fat pointer in another box, leading to this double
        // indirection.
        let cb: Box<Box<dyn Fn(Point<P>, Point<P>, Point<P>, Point<P>, &mut Point<P>)>> =
            Box::new(Box::new(callback));
        let raw_cb = Box::into_raw(cb) as *mut _;

        // It there is an old callback stored, drop it before replacing it
        if let Some(old_raw_cb) = self.raw_z_callback {
            drop(unsafe { Box::from_raw(old_raw_cb as *mut _) });
        }
        self.raw_z_callback = Some(raw_cb);

        unsafe {
            clipper2c_sys::clipper_clipper64_set_z_callback(
                self.ptr,
                raw_cb,
                Some(handle_set_z_callback::<P>),
            );
        }
    }
}

#[cfg(feature = "usingz")]
extern "C" fn handle_set_z_callback<P: PointScaler>(
    user_data: *mut ::std::os::raw::c_void,
    e1bot: *const clipper2c_sys::ClipperPoint64,
    e1top: *const clipper2c_sys::ClipperPoint64,
    e2bot: *const clipper2c_sys::ClipperPoint64,
    e2top: *const clipper2c_sys::ClipperPoint64,
    pt: *mut clipper2c_sys::ClipperPoint64,
) {
    use crate::Point;

    // SAFETY: user_data was set in set_z_callback, and is valid for as long as
    // the Clipper2 instance exists.
    let callback: &mut &mut dyn Fn(Point<P>, Point<P>, Point<P>, Point<P>, &mut crate::Point<P>) =
        unsafe { std::mem::transmute(user_data) };

    // SAFETY: Clipper2 should produce valid pointers
    let mut new_point = unsafe { Point::<P>::from(*pt) };
    let e1bot = unsafe { Point::<P>::from(*e1bot) };
    let e1top = unsafe { Point::<P>::from(*e1top) };
    let e2bot = unsafe { Point::<P>::from(*e2bot) };
    let e2top = unsafe { Point::<P>::from(*e2top) };

    callback(e1bot, e1top, e2bot, e2top, &mut new_point);

    // SAFETY: pt is a valid pointer and new_point is not null
    unsafe {
        *pt = *new_point.as_clipperpoint64();
    }
}

impl<P: PointScaler> Clipper<WithSubjects, P> {
    /// Adds another subject path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path).add_subject(path2);
    /// ```
    pub fn add_subject(self, subject: impl Into<Paths<P>>) -> Self {
        unsafe {
            let subject_ptr = subject.into().to_clipperpaths64();
            clipper_clipper64_add_subject(self.ptr, subject_ptr);
            clipper_delete_paths64(subject_ptr);
        }

        self
    }

    /// Adds another open subject path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path).add_open_subject(path2);
    /// ```
    pub fn add_open_subject(self, subject: impl Into<Paths<P>>) -> Self {
        unsafe {
            let subject_ptr = subject.into().to_clipperpaths64();
            clipper_clipper64_add_open_subject(self.ptr, subject_ptr);
            clipper_delete_paths64(subject_ptr);
        }

        self
    }

    /// Adds a clip path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path).add_clip(path2);
    /// ```
    pub fn add_clip(mut self, clip: impl Into<Paths<P>>) -> Clipper<WithClips, P> {
        self.keep_ptr_on_drop = true;

        let clipper = Clipper::<WithClips, P> {
            ptr: self.ptr,
            keep_ptr_on_drop: false,
            _marker: PhantomData,
            _state: WithClips {},

            #[cfg(feature = "usingz")]
            raw_z_callback: None,
        };

        drop(self);

        clipper.add_clip(clip)
    }
}

impl<P: PointScaler> Clipper<WithClips, P> {
    /// Adds another clip path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    /// let path3: Paths = vec![(2.2, 2.2), (5.0, 2.2), (2.2, 5.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path).add_clip(path2).add_clip(path3);
    /// ```
    pub fn add_clip(self, clip: impl Into<Paths<P>>) -> Self {
        unsafe {
            let clip_ptr = clip.into().to_clipperpaths64();
            clipper_clipper64_add_clip(self.ptr, clip_ptr);
            clipper_delete_paths64(clip_ptr);
        }

        self
    }

    /// Applies a union boolean operation to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new().add_subject(path).add_clip(path2).union(FillRule::NonZero);
    /// ```
    ///
    /// For more details see the original [union](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Union.htm) docs.
    pub fn union(self, fill_rule: FillRule) -> Result<Paths<P>, ClipperError> {
        self.boolean_operation(ClipType::Union, fill_rule)
    }

    /// Applies a difference boolean operation to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new().add_subject(path).add_clip(path2).difference(FillRule::NonZero);
    /// ```
    ///
    /// For more details see the original [difference](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Difference.htm) docs.
    pub fn difference(self, fill_rule: FillRule) -> Result<Paths<P>, ClipperError> {
        self.boolean_operation(ClipType::Difference, fill_rule)
    }

    /// Applies an intersection boolean operation to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new().add_subject(path).add_clip(path2).intersect(FillRule::NonZero);
    /// ```
    ///
    /// For more details see the original [intersect](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Intersect.htm) docs.
    pub fn intersect(self, fill_rule: FillRule) -> Result<Paths<P>, ClipperError> {
        self.boolean_operation(ClipType::Intersection, fill_rule)
    }

    /// Applies an xor boolean operation to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new().add_subject(path).add_clip(path2).xor(FillRule::NonZero);
    /// ```
    ///
    /// For more details see the original [xor](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/XOR.htm) docs.
    pub fn xor(self, fill_rule: FillRule) -> Result<Paths<P>, ClipperError> {
        self.boolean_operation(ClipType::Xor, fill_rule)
    }

    fn boolean_operation(
        self,
        clip_type: ClipType,
        fill_rule: FillRule,
    ) -> Result<Paths<P>, ClipperError> {
        let closed_path = unsafe { Paths::<P>::new(Vec::new()).to_clipperpaths64() };
        let open_path = unsafe { Paths::<P>::new(Vec::new()).to_clipperpaths64() };

        let result = unsafe {
            let success = clipper_clipper64_execute(
                self.ptr,
                clip_type.into(),
                fill_rule.into(),
                closed_path,
                open_path,
            );

            if success != 1 {
                clipper_delete_paths64(closed_path);
                clipper_delete_paths64(open_path);
                return Err(ClipperError::FailedBooleanOperation);
            }

            let path = Paths::from_clipperpaths64(closed_path);
            clipper_delete_paths64(closed_path);
            clipper_delete_paths64(open_path);

            Ok(path)
        };

        drop(self);

        result
    }
}

impl Default for Clipper<NoSubjects, Centi> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: ClipperState, P: PointScaler> Drop for Clipper<S, P> {
    fn drop(&mut self) {
        if !self.keep_ptr_on_drop {
            unsafe { clipper_delete_clipper64(self.ptr) }

            #[cfg(feature = "usingz")]
            {
                if let Some(raw_cb) = self.raw_z_callback {
                    drop(unsafe { Box::from_raw(raw_cb as *mut _) });
                }
            }
        }
    }
}

/// Errors that can occur during clipper operations.
#[derive(Debug, thiserror::Error)]
pub enum ClipperError {
    /// Failed execute boolean operation.
    #[error("Failed boolean operation")]
    FailedBooleanOperation,
}

#[cfg(test)]
mod test {
    #[cfg(feature = "usingz")]
    #[test]
    fn test_set_z_callback() {
        use std::{cell::Cell, rc::Rc};

        use super::*;
        use crate::Point;

        let mut clipper = Clipper::<NoSubjects, Centi>::new();
        let success = Rc::new(Cell::new(false));
        {
            let success = success.clone();
            clipper.set_z_callback(
                move |_e1bot: Point<_>,
                      _e1top: Point<_>,
                      _e2bot: Point<_>,
                      _e2top: Point<_>,
                      p: &mut Point<_>| {
                    p.set_z(1);
                    success.set(true);
                },
            );
        }
        let e1: Paths = vec![(0.0, 0.0), (2.0, 2.0), (0.0, 2.0)].into();
        let e2: Paths = vec![(1.0, 0.0), (2.0, 0.0), (1.0, 2.0)].into();
        let paths = clipper
            .add_subject(e1)
            .add_clip(e2)
            .union(FillRule::default())
            .unwrap();

        assert_eq!(success.get(), true);

        let n_intersecting = paths
            .iter()
            .flatten()
            .into_iter()
            .filter(|v| v.z() == 1)
            .count();
        assert_eq!(n_intersecting, 3);
    }
}
