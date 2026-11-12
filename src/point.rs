use std::marker::PhantomData;

use clipper2c_sys::ClipperPoint64;

/// The point scaling trait allows to choose a multiplier for the point values
/// at compile time.
///
/// The default multiplier is `Centi`, and others are provided by the library,
/// but if needed the user can create a custom scaler struct that implements
/// `PointScaler`.
///
/// # Choosing a scaler
///
/// A point is stored as `round(value * MULTIPLIER)` in an `i64`, so the
/// multiplier sets both the resolution and the range. Clipper2 requires
/// scaled coordinates to stay within `+-(i64::MAX >> 2)` (about `2.3e18`) for
/// its intersection arithmetic to be exact, so the usable range is that limit
/// divided by the multiplier. The table gives the resolution in input units
/// and the largest coordinate magnitude that fits, both for Clipper2's limit
/// and for the plain `i64` limit.
///
/// | Scaler  | Multiplier | Resolution | Range (Clipper2 limit) | Range (`i64`) |
/// |---------|-----------:|-----------:|-----------------------:|--------------:|
/// | `One`   | 1          | 1          | +-2.3e18               | +-9.2e18      |
/// | `Deci`  | 1e1        | 0.1        | +-2.3e17               | +-9.2e17      |
/// | `Centi` | 1e2        | 0.01       | +-2.3e16               | +-9.2e16      |
/// | `Milli` | 1e3        | 0.001      | +-2.3e15               | +-9.2e15      |
/// | `Micro` | 1e6        | 0.000001   | +-2.3e12               | +-9.2e12      |
/// | `Nano`  | 1e9        | 0.000000001| +-2.3e9                | +-9.2e9       |
///
/// With millimetres as the input unit, `Milli` resolves micrometres, `Micro`
/// resolves nanometres and `Nano` resolves picometres, the latter covering
/// coordinates up to about 2300 km.
pub trait PointScaler: Default + Clone + Copy + PartialEq + std::hash::Hash {
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
#[derive(Debug, Default, Copy, Clone, PartialEq, Hash)]
pub struct One;

impl PointScaler for One {
    const MULTIPLIER: f64 = 1.0;
}

/// Scale by 10.
#[derive(Debug, Default, Copy, Clone, PartialEq, Hash)]
pub struct Deci;

impl PointScaler for Deci {
    const MULTIPLIER: f64 = 10.0;
}

/// Scale by 100. This is the default.
#[derive(Debug, Default, Copy, Clone, PartialEq, Hash)]
pub struct Centi;

impl PointScaler for Centi {
    const MULTIPLIER: f64 = 100.0;
}

/// Scale by 1000.
#[derive(Debug, Default, Copy, Clone, PartialEq, Hash)]
pub struct Milli;

impl PointScaler for Milli {
    const MULTIPLIER: f64 = 1000.0;
}

/// Scale by 1,000,000.
#[derive(Debug, Default, Copy, Clone, PartialEq, Hash)]
pub struct Micro;

impl PointScaler for Micro {
    const MULTIPLIER: f64 = 1_000_000.0;
}

/// Scale by 1,000,000,000.
#[derive(Debug, Default, Copy, Clone, PartialEq, Hash)]
pub struct Nano;

impl PointScaler for Nano {
    const MULTIPLIER: f64 = 1_000_000_000.0;
}

/// XY Point with custom scaler.
///
/// For
/// [rubustness reasons](https://www.angusj.com/clipper2/Docs/Robustness.htm)
/// clipper2 uses 64bit integers to store coordinates.
///
/// Therefore you can choose a implementation of PointScaler for your
/// use-case. This library offers `One`, `Deci`, `Centi`, `Milli`, `Micro` and `Nano`
/// multipliers (see [`PointScaler`] for a table of resolution and range) where `Centi` is the default (multiplies values by 100 when converting to
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
///
/// let point = Point::<Micro>::new(1.0, 2.0);
/// assert_eq!(point.x_scaled(), 1_000_000);
/// assert_eq!(point.y_scaled(), 2_000_000);
///
/// let point = Point::<Nano>::new(1.0, 2.0);
/// assert_eq!(point.x_scaled(), 1_000_000_000);
/// assert_eq!(point.y_scaled(), 2_000_000_000);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent),
    serde(bound = "P: PointScaler")
)]
pub struct Point<P: PointScaler = Centi>(
    ClipperPoint64,
    #[cfg_attr(feature = "serde", serde(skip))] PhantomData<P>,
);

impl<P: PointScaler> Point<P> {
    /// The zero point.
    pub const ZERO: Self = Self(ClipperPoint64 { x: 0, y: 0 }, PhantomData);

    /// The minimum value for a point.
    pub const MIN: Self = Self(
        ClipperPoint64 {
            x: i64::MIN,
            y: i64::MIN,
        },
        PhantomData,
    );

    /// The maximum value for a point.
    pub const MAX: Self = Self(
        ClipperPoint64 {
            x: i64::MAX,
            y: i64::MAX,
        },
        PhantomData,
    );

    /// Create a new point. The coordinates are scaled and rounded to the
    /// integer grid of `P`: NaN becomes 0, and infinite or out-of-range values
    /// saturate to the grid's limits, where boolean results are not reliable.
    pub fn new(x: f64, y: f64) -> Self {
        Self(
            ClipperPoint64 {
                x: P::scale(x).round() as i64,
                y: P::scale(y).round() as i64,
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

    /// Calculate the distance to another point.
    pub fn distance_to(&self, to: &Self) -> f64 {
        ((self.x() - to.x()).powf(2.0) + (self.y() - to.y()).powf(2.0)).sqrt()
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
    fn test_point_multiplier_rounding() {
        let point = Point::<Centi>::new(2.05, 2.125);
        assert_eq!(point.x_scaled(), 205);
        assert_eq!(point.y_scaled(), 213);
    }

    #[test]
    fn test_point_micro_and_nano() {
        let point = Point::<Micro>::new(1.5, -2.000_000_4);
        assert_eq!(point.x_scaled(), 1_500_000);
        assert_eq!(point.y_scaled(), -2_000_000);
        assert_eq!(point.x(), 1.5);

        let point = Point::<Nano>::new(1.5, -2.000_000_000_4);
        assert_eq!(point.x_scaled(), 1_500_000_000);
        assert_eq!(point.y_scaled(), -2_000_000_000);
        assert_eq!(point.x(), 1.5);
    }

    #[test]
    fn test_scaler_multipliers() {
        assert_eq!(One::MULTIPLIER, 1.0);
        assert_eq!(Deci::MULTIPLIER, 10.0);
        assert_eq!(Centi::MULTIPLIER, 100.0);
        assert_eq!(Milli::MULTIPLIER, 1e3);
        assert_eq!(Micro::MULTIPLIER, 1e6);
        assert_eq!(Nano::MULTIPLIER, 1e9);
    }

    #[test]
    fn test_nano_boolean_operation_resolves_small_features() {
        let a: crate::Paths<Nano> = vec![
            (0.0, 0.0),
            (0.000_003, 0.0),
            (0.000_003, 0.000_003),
            (0.0, 0.000_003),
        ]
        .into();
        let b: crate::Paths<Nano> = vec![
            (0.000_001, 0.0),
            (0.000_005, 0.0),
            (0.000_005, 0.000_003),
            (0.000_001, 0.000_003),
        ]
        .into();
        let result = crate::union(a, b, crate::FillRule::NonZero).unwrap();
        assert_eq!(result.len(), 1);
        assert!((result.signed_area() - 15e-12).abs() < 1e-15);
    }

    #[test]
    fn test_point_custom_scaler() {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
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

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let point = Point::<Centi>::new(1.0, 2.0);
        let serialized = serde_json::to_string(&point).unwrap();
        assert_eq!(serialized, r#"{"x":100,"y":200}"#);

        let deserialized: Point<Centi> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(point, deserialized);
    }

    #[test]
    fn test_distance_to() {
        let point1 = Point::<Centi>::new(1.0, 2.0);
        let point2 = Point::<Centi>::new(3.0, 4.0);
        assert_eq!(point1.distance_to(&point2), 2.8284271247461903);
    }
}
