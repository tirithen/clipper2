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
        };

        drop(self);

        clipper.add_open_subject(subject)
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
    /// let clipper = Clipper::new().add_subject(path).add_clip(path2).union(FillRule::NonZero);
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
    /// let clipper = Clipper::new().add_subject(path).add_clip(path2).difference(FillRule::NonZero);
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
    /// let clipper = Clipper::new().add_subject(path).add_clip(path2).intersect(FillRule::NonZero);
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
    /// let clipper = Clipper::new().add_subject(path).add_clip(path2).xor(FillRule::NonZero);
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
