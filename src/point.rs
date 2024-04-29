use std::marker::PhantomData;

use clipper2c_sys::ClipperPoint64;

/// The point scaling trait allows to choose a multiplier for the point values
/// at compile time.
///
/// The default multiplier is `Centi`, and others are provided by the library,
/// but if needed the user can create a custom scaler struct that implements
/// `PointScaler`.
pub trait PointScaler: Clone + Copy {
    /// The point multiplier. This is set to a custom value when implementing
    /// the `PointScaler` trait.
    const MULTIPLIER: f64;

    /// Scale a value by the multiplier.
    fn scale(value: f64) -> f64 {
        value * Self::MULTIPLIER
    }

    /// Descale/unscale a value by the multiplier.
    fn descale(value: f64) -> f64 {
        value / Self::MULTIPLIER
    }
}

/// No scaling.
#[derive(Debug, Copy, Clone)]
pub struct One;

impl PointScaler for One {
    const MULTIPLIER: f64 = 1.0;
}

/// Scale by 10.
#[derive(Debug, Copy, Clone)]
pub struct Deci;

impl PointScaler for Deci {
    const MULTIPLIER: f64 = 10.0;
}

/// Scale by 100. This is the default.
#[derive(Debug, Copy, Clone)]
pub struct Centi;

impl PointScaler for Centi {
    const MULTIPLIER: f64 = 100.0;
}

/// Scale by 1000.
#[derive(Debug, Copy, Clone)]
pub struct Milli;

impl PointScaler for Milli {
    const MULTIPLIER: f64 = 1000.0;
}

/// XY Point with custom scaler.
///
/// For
/// [rubustness reasons](https://www.angusj.com/clipper2/Docs/Robustness.htm)
/// clipper2 uses 64bit integers to store coordinates.
///
/// Therefore you can choose a implementation of PointScaler for your
/// use-case. This library offers `One`, `Deci`, `Centi` and `Milli` multipliers
/// where `Centi` is the default (multiplies values by 100 when converting to
/// i64).
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let point = Point::<Centi>::new(1.0, 2.0);
/// assert_eq!(point.x(), 1.0);
/// assert_eq!(point.y(), 2.0);
/// assert_eq!(point.x_scaled(), 100);
/// assert_eq!(point.y_scaled(), 200);
///
/// let point = Point::<Milli>::new(1.0, 2.0);
/// assert_eq!(point.x(), 1.0);
/// assert_eq!(point.y(), 2.0);
/// assert_eq!(point.x_scaled(), 1000);
/// assert_eq!(point.y_scaled(), 2000);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Point<P: PointScaler = Centi>(ClipperPoint64, PhantomData<P>);

impl<P: PointScaler> Point<P> {
    /// The zero point.
    pub const ZERO: Self = Self(ClipperPoint64 { x: 0, y: 0 }, PhantomData);

    /// Create a new point.
    pub fn new(x: f64, y: f64) -> Self {
        Self(
            ClipperPoint64 {
                x: P::scale(x) as i64,
                y: P::scale(y) as i64,
            },
            PhantomData,
        )
    }

    /// Create a new point from scaled values, this means that point is
    /// constructed as is without applying the scaling multiplier.
    pub fn from_scaled(x: i64, y: i64) -> Self {
        Self(ClipperPoint64 { x, y }, PhantomData)
    }

    /// Returns the x coordinate of the point.
    pub fn x(&self) -> f64 {
        P::descale(self.0.x as f64)
    }

    /// Returns the y coordinate of the point.
    pub fn y(&self) -> f64 {
        P::descale(self.0.y as f64)
    }

    /// Returns the scaled x coordinate of the point.
    pub fn x_scaled(&self) -> i64 {
        self.0.x
    }

    /// Returns the scaled y coordinate of the point.
    pub fn y_scaled(&self) -> i64 {
        self.0.y
    }

    pub(crate) fn as_clipperpoint64(&self) -> *const ClipperPoint64 {
        &self.0
    }
}

impl<P: PointScaler> Default for Point<P> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<P: PointScaler> From<ClipperPoint64> for Point<P> {
    fn from(point: ClipperPoint64) -> Self {
        Self(point, PhantomData)
    }
}

impl<P: PointScaler> From<Point<P>> for ClipperPoint64 {
    fn from(point: Point<P>) -> Self {
        point.0
    }
}

impl<P: PointScaler> From<(f64, f64)> for Point<P> {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl<P: PointScaler> From<&(f64, f64)> for Point<P> {
    fn from((x, y): &(f64, f64)) -> Self {
        Self::new(*x, *y)
    }
}

impl<P: PointScaler> From<[f64; 2]> for Point<P> {
    fn from([x, y]: [f64; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<P: PointScaler> From<&[f64; 2]> for Point<P> {
    fn from([x, y]: &[f64; 2]) -> Self {
        Self::new(*x, *y)
    }
}

impl<P: PointScaler> From<Point<P>> for (f64, f64) {
    fn from(point: Point<P>) -> Self {
        (point.x(), point.y())
    }
}

impl<P: PointScaler> From<Point<P>> for [f64; 2] {
    fn from(point: Point<P>) -> Self {
        [point.x(), point.y()]
    }
}

impl<P: PointScaler> PartialEq for Point<P> {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_default_multiplier() {
        let point = Point::<Centi>::new(1.0, 2.0);
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.x_scaled(), 100);
        assert_eq!(point.y_scaled(), 200);
    }

    #[test]
    fn test_point_custom_scaler() {
        #[derive(Debug, Copy, Clone)]
        struct CustomScaler;

        impl PointScaler for CustomScaler {
            const MULTIPLIER: f64 = 2000.0;
        }

        let point = Point::<CustomScaler>::new(1.0, 2.0);
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.x_scaled(), 2000);
        assert_eq!(point.y_scaled(), 4000);
    }
}
