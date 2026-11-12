use std::marker::PhantomData;

use clipper2c_sys::{
    ClipperClipper64, clipper_clipper64, clipper_clipper64_add_clip,
    clipper_clipper64_add_open_subject, clipper_clipper64_add_subject, clipper_clipper64_execute,
    clipper_clipper64_size, clipper_delete_clipper64, clipper_delete_paths64,
};

use crate::{Centi, ClipType, FillRule, Paths, PointScaler, malloc};

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
///
/// # Output orientation
///
/// The paths returned by the boolean operations follow Clipper2's default
/// orientation: outer paths run counter-clockwise and holes run clockwise.
/// This holds whatever the orientation of the input paths. With the usual
/// Cartesian axes (Y up) outer paths therefore have a positive
/// [`crate::Path::signed_area`] and holes a negative one, so [`crate::Paths::signed_area`] of
/// a result is the area of the filled region with the holes taken off. With Y
/// pointing down, as in most screen coordinate systems, the visual directions
/// and the signs are mirrored, but outer paths still have the positive area.
///
/// # Rounding to the scaler grid
///
/// Clipper2 works on 64-bit integers. Every input coordinate is rounded to the
/// nearest step of the [`PointScaler`] grid when a [`crate::Point`] is created (for
/// example 0.01 for the default [`Centi`]), and the points where edges
/// intersect are rounded to that grid as well. A boolean operation is
/// therefore not the identity on input that is not on the grid: the result
/// has its coordinates moved to the grid, and vertices that become redundant
/// (collinear or duplicate) can be dropped or the start point and order of the
/// paths can change, even if the operation changes nothing else. Choose a
/// scaler with a resolution finer than the accuracy you need, see
/// [`PointScaler`] for a table of resolutions and ranges.
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
    /// The boolean operations return the closed paths of the result only:
    /// what is left of open subjects is not returned.
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
    /// Applies a union boolean operation to the subjects alone, without any
    /// clip paths.
    ///
    /// This resolves overlapping and self-intersecting subject paths under the
    /// given fill rule. For example, rings that are filled according to
    /// [`FillRule::NonZero`] come out as outer paths and holes, and
    /// [`FillRule::EvenOdd`] turns nested rings into holes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let outer: Paths = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)].into();
    /// let inner: Paths = vec![(2.0, 2.0), (8.0, 2.0), (8.0, 8.0), (2.0, 8.0)].into();
    ///
    /// let result = Clipper::new()
    ///     .add_subject(outer)
    ///     .add_subject(inner)
    ///     .union(FillRule::EvenOdd)
    ///     .expect("Failed union operation");
    ///
    /// // The inner ring is a hole.
    /// assert_eq!(result.len(), 2);
    /// assert_eq!(result.signed_area(), 64.0);
    /// ```
    ///
    /// For more details see the original [union](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Union.htm) docs.
    pub fn union(self, fill_rule: FillRule) -> Result<Paths<P>, ClipperError> {
        self.boolean_operation(ClipType::Union, fill_rule)
    }

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
    /// The boolean operations return the closed paths of the result only:
    /// what is left of open subjects is not returned.
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
}

impl<S: ClipperState, P: PointScaler> Clipper<S, P> {
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
    ///
    /// This is returned when the Clipper2 engine reports that `Execute`
    /// failed. In the C++ source this flag (`succeeded_`) is cleared only when
    /// the sweep reaches an inconsistent state, namely when two edges meet at
    /// a local maximum with the same front/back orientation although neither
    /// is the end of an open path. That cannot happen for well-formed input
    /// within the coordinate limits: empty input, open paths, self-intersecting
    /// paths and overlapping paths are all handled and do not cause it.
    ///
    /// The engine does not check the range of its input, so the practical
    /// cause is scaled coordinates outside the range that Clipper2 supports
    /// (magnitudes above `i64::MAX >> 2`, about 2.3e18, see the table in
    /// [`PointScaler`]), where the integer intersection arithmetic no longer
    /// holds. The error carries no further detail and does not say which input
    /// path caused it.
    #[error("Failed boolean operation")]
    FailedBooleanOperation,
}

#[cfg(test)]
mod test {
    use crate::{Centi, FillRule, Milli, Paths, Point};

    fn ring_area(paths: &Paths) -> Vec<f64> {
        paths.iter().map(|path| path.signed_area()).collect()
    }

    #[test]
    fn test_outer_is_counter_clockwise_and_hole_is_clockwise() {
        let outer = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        let hole = vec![(2.0, 2.0), (8.0, 2.0), (8.0, 8.0), (2.0, 8.0)];

        for (outer, hole) in [
            (outer.clone(), hole.clone()),
            (
                outer.iter().rev().copied().collect::<Vec<_>>(),
                hole.iter().rev().copied().collect::<Vec<_>>(),
            ),
            (outer.clone(), hole.iter().rev().copied().collect()),
        ] {
            let subject: Paths = vec![outer].into();
            let clip: Paths = vec![hole].into();
            let result = subject
                .to_clipper_subject()
                .add_clip(clip)
                .difference(FillRule::NonZero);
            let mut areas = ring_area(&result.unwrap());
            areas.sort_by(|a, b| a.partial_cmp(b).unwrap());
            assert_eq!(areas, vec![-36.0, 100.0]);
        }
    }

    #[test]
    fn test_union_of_clockwise_input_comes_out_counter_clockwise() {
        let clockwise: Paths =
            vec![vec![(0.0, 0.0), (0.0, 10.0), (10.0, 10.0), (10.0, 0.0)]].into();
        assert_eq!(clockwise.signed_area(), -100.0);
        let result = clockwise.union(FillRule::NonZero).unwrap();
        assert_eq!(result.signed_area(), 100.0);
    }

    #[test]
    fn test_off_grid_input_is_rounded_to_the_grid() {
        let paths: Paths<Centi> = vec![vec![
            (0.001, 0.004),
            (10.006, 0.0),
            (10.0, 10.0),
            (0.0, 10.0),
        ]]
        .into();
        let result = paths.union(FillRule::NonZero).unwrap();
        for path in result.iter() {
            for point in path.iter() {
                assert_eq!(point.x(), (point.x() * 100.0).round() / 100.0);
                assert_eq!(point.y(), (point.y() * 100.0).round() / 100.0);
            }
        }
        let has_rounded_corner = result
            .iter()
            .flat_map(|path| path.iter())
            .any(|point| point.x_scaled() == 1001 && point.y_scaled() == 0);
        assert!(has_rounded_corner);
    }

    #[test]
    fn test_finer_scaler_keeps_detail_that_a_coarser_one_rounds_away() {
        let coarse: Paths<Centi> = vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 0.004)]].into();
        let fine: Paths<Milli> = vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 0.004)]].into();
        assert_eq!(coarse.union(FillRule::NonZero).unwrap().signed_area(), 0.0);
        assert!(fine.union(FillRule::NonZero).unwrap().signed_area() > 0.0);
        assert_eq!(Point::<Milli>::new(1.0, 0.004).y_scaled(), 4);
    }
}
