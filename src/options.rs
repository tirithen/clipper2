#![allow(non_upper_case_globals)]

use clipper2c_sys::{
    ClipperClipType, ClipperClipType_DIFFERENCE, ClipperClipType_INTERSECTION,
    ClipperClipType_NONE, ClipperClipType_UNION, ClipperClipType_XOR, ClipperEndType,
    ClipperEndType_BUTT_END, ClipperEndType_JOINED_END, ClipperEndType_POLYGON_END,
    ClipperEndType_ROUND_END, ClipperEndType_SQUARE_END, ClipperFillRule, ClipperFillRule_EVEN_ODD,
    ClipperFillRule_NEGATIVE, ClipperFillRule_NON_ZERO, ClipperFillRule_POSITIVE, ClipperJoinType,
    ClipperJoinType_BEVEL_JOIN, ClipperJoinType_MITER_JOIN, ClipperJoinType_ROUND_JOIN,
    ClipperJoinType_SQUARE_JOIN, ClipperPathType, ClipperPathType_CLIP, ClipperPathType_SUBJECT,
    ClipperPointInPolygonResult, ClipperPointInPolygonResult_IS_INSIDE,
    ClipperPointInPolygonResult_IS_ON, ClipperPointInPolygonResult_IS_OUTSIDE,
};

/// The Clipper Library supports 4 filling rules: Even-Odd, Non-Zero, Positive
/// and Negative. These rules are base on the winding numbers (see below) of
/// each polygon sub-region, which in turn are based on the orientation of each
/// path. Orientation is determined by the order in which vertices are declared
/// during path construction, and whether these vertices progress roughly
/// clockwise or counter-clockwise.
///
/// Winding numbers for polygon sub-regions can be derived using the following
/// algorithm:
///
/// * From a point that's outside a given polygon, draw an imaginary line
///   through the polygon.
/// * Starting with a winding number of zero, and beginning at the start of this
///   imaginary line, follow the line as it crosses the polygon.
/// * For each polygon contour that you cross, increment the winding number if
///   the line is heading right to left (relative to your imaginary line),
///   otherwise decrement the winding number.
/// * And as you enter each polygon sub-region, allocate to it the current
///   winding number.
///
/// * Even-Odd: Only odd numbered sub-regions are filled
/// * Non-Zero: Only non-zero sub-regions are filled
/// * Positive: Only sub-regions with winding counts > 0 are filled
/// * Negative: Only sub-regions with winding counts < 0 are filled
///
/// For more details see [FillRule](https://www.angusj.com/clipper2/Docs/Units/Clipper/Types/FillRule.htm).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum FillRule {
    /// Even-Odd filling rule
    #[default]
    EvenOdd,
    /// Non-Zero filling rule
    NonZero,
    /// Positive filling rule
    Positive,
    /// Negative filling rule
    Negative,
}

/// There are four boolean operations:
///
/// * AND (intersection) - regions covered by both subject and clip polygons
/// * OR (union) - regions covered by subject or clip polygons, or both polygons
/// * NOT (difference) - regions covered by subject, but not clip polygons
/// * XOR (exclusive or) - regions covered by subject or clip polygons, but not
///   both
///
/// Of these four operations, only difference is non-commutative. This means
/// that subject and clip paths are interchangeable except when performing
/// difference operations (and as long as subject paths are closed).
///
/// For more details see [ClipType](https://www.angusj.com/clipper2/Docs/Units/Clipper/Types/ClipType.htm).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ClipType {
    None,
    Intersection,
    Union,
    Difference,
    Xor,
}

/// The JoinType enumeration is only needed when offsetting
/// (inflating/shrinking). It isn't needed for polygon clipping. It specifies
/// how to manage offsetting at convex angled joins. Concave joins will always
/// be offset with a mitered join.
///
/// When adding paths to a ClipperOffset object via the AddPaths method, the
/// JoinType parameter must specify one of the following types - Miter, Square,
/// Bevel or Round.
///
/// For more details see [JoinType](https://www.angusj.com/clipper2/Docs/Units/Clipper/Types/JoinType.htm).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoinType {
    /// Square join
    Square,
    /// Bevel join
    Bevel,
    /// Round join
    Round,
    /// Miter join
    Miter,
}

/// The EndType enumerator is only needed when offsetting (inflating/shrinking).
/// It isn't needed for polygon clipping.
///
/// EndType has 5 values:
///
/// * Polygon: the path is treated as a polygon
/// * Joined: ends are joined and the path treated as a polyline
/// * Butt: ends are squared off without any extension
/// * Square: ends extend the offset amount while being squared off
/// * Round: ends extend the offset amount while being rounded off
///
/// With both EndType.Polygon and EndType.Joined, path closure will occur
/// regardless of whether or not the first and last vertices in the path match.
///
/// For more details see [EndType](https://www.angusj.com/clipper2/Docs/Units/Clipper/Types/EndType.htm).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EndType {
    /// Polygon: the path is treated as a polygon
    Polygon,
    /// Joined: ends are joined and the path treated as a polyline
    Joined,
    /// Butt: ends are squared off without any extension
    Butt,
    /// Square: ends extend the offset amount while being squared off
    Square,
    /// Round: ends extend the offset amount while being rounded off
    Round,
}

/// The result indicates whether the point is inside, or outside, or on one of
/// the specified polygon's edges.
///
/// * IsOn: the point is on the path
/// * IsInside: the point is inside the path
/// * IsOutside: the point is outside the path
///
/// For more details see [PointInPolygonResult](https://www.angusj.com/clipper2/Docs/Units/Clipper/Types/PointInPolygonResult.htm).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointInPolygonResult {
    /// The point is on the path
    IsOn,
    /// The point is inside the path
    IsInside,
    /// The point is outside the path
    IsOutside,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PathType {
    Subject,
    Clip,
}

impl From<ClipType> for ClipperClipType {
    fn from(value: ClipType) -> Self {
        match value {
            ClipType::None => ClipperClipType_NONE,
            ClipType::Intersection => ClipperClipType_INTERSECTION,
            ClipType::Union => ClipperClipType_UNION,
            ClipType::Difference => ClipperClipType_DIFFERENCE,
            ClipType::Xor => ClipperClipType_XOR,
        }
    }
}

impl From<FillRule> for ClipperFillRule {
    fn from(value: FillRule) -> Self {
        match value {
            FillRule::EvenOdd => ClipperFillRule_EVEN_ODD,
            FillRule::NonZero => ClipperFillRule_NON_ZERO,
            FillRule::Positive => ClipperFillRule_POSITIVE,
            FillRule::Negative => ClipperFillRule_NEGATIVE,
        }
    }
}

impl From<JoinType> for ClipperJoinType {
    fn from(value: JoinType) -> Self {
        match value {
            JoinType::Square => ClipperJoinType_SQUARE_JOIN,
            JoinType::Bevel => ClipperJoinType_BEVEL_JOIN,
            JoinType::Round => ClipperJoinType_ROUND_JOIN,
            JoinType::Miter => ClipperJoinType_MITER_JOIN,
        }
    }
}

impl From<EndType> for ClipperEndType {
    fn from(value: EndType) -> Self {
        match value {
            EndType::Polygon => ClipperEndType_POLYGON_END,
            EndType::Joined => ClipperEndType_JOINED_END,
            EndType::Butt => ClipperEndType_BUTT_END,
            EndType::Square => ClipperEndType_SQUARE_END,
            EndType::Round => ClipperEndType_ROUND_END,
        }
    }
}

impl From<PointInPolygonResult> for ClipperPointInPolygonResult {
    fn from(value: PointInPolygonResult) -> Self {
        match value {
            PointInPolygonResult::IsOn => ClipperPointInPolygonResult_IS_ON,
            PointInPolygonResult::IsInside => ClipperPointInPolygonResult_IS_INSIDE,
            PointInPolygonResult::IsOutside => ClipperPointInPolygonResult_IS_OUTSIDE,
        }
    }
}

impl From<ClipperPointInPolygonResult> for PointInPolygonResult {
    fn from(value: ClipperPointInPolygonResult) -> Self {
        match value {
            ClipperPointInPolygonResult_IS_ON => PointInPolygonResult::IsOn,
            ClipperPointInPolygonResult_IS_INSIDE => PointInPolygonResult::IsInside,
            ClipperPointInPolygonResult_IS_OUTSIDE => PointInPolygonResult::IsOutside,
            _ => panic!(
                "Invalid ClipperPointInPolygonResult value {}",
                value as usize
            ),
        }
    }
}

impl From<PathType> for ClipperPathType {
    fn from(value: PathType) -> Self {
        match value {
            PathType::Subject => ClipperPathType_SUBJECT,
            PathType::Clip => ClipperPathType_CLIP,
        }
    }
}
