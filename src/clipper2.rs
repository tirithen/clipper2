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

// pub fn inflate(
//     polygons: Polygons,
//     delta: f64,
//     join_type: JoinType,
//     end_type: EndType,
//     miter_limit: f64,
//     arc_tolerance: f64,
// ) -> Polygons {
//     let precision = PRECISION_MULTIPLIER;

//     unsafe {
//         inflate_c(
//             polygons.into(),
//             delta * precision,
//             join_type.into(),
//             end_type.into(),
//             miter_limit * precision,
//             arc_tolerance * precision,
//         )
//         .into()
//     }
// }

// pub fn intersect(subjects: Polygons, clips: Polygons) -> Polygons {
//     unsafe { intersect_c(subjects.into(), clips.into()).into() }
// }

pub fn union(subjects: Paths, fill_rule: FillRule) -> Paths {
    unsafe { union_c(subjects, fill_rule.into()).into() }
}

// pub fn difference(subjects: Polygons, clips: Polygons) -> Polygons {
//     unsafe { difference_c(subjects.into(), clips.into()).into() }
// }

// pub fn xor(subjects: Polygons, clips: Polygons) -> Polygons {
//     unsafe { xor_c(subjects.into(), clips.into()).into() }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vertex(i64, i64);

impl Vertex {
    pub fn new(x: f64, y: f64) -> Self {
        Self(
            (x * PRECISION_MULTIPLIER) as i64,
            (y * PRECISION_MULTIPLIER) as i64,
        )
    }

    pub fn x(&self) -> f64 {
        self.0 as f64 / PRECISION_MULTIPLIER
    }

    pub fn y(&self) -> f64 {
        self.1 as f64 / PRECISION_MULTIPLIER
    }
}

impl From<Vertex> for [f64; 2] {
    fn from(value: Vertex) -> [f64; 2] {
        [value.x(), value.y()]
    }
}

impl From<[f64; 2]> for Vertex {
    fn from(value: [f64; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}

#[cfg(feature = "glam")]
impl From<Vertex> for glam::Vec2 {
    fn from(value: Vertex) -> Self {
        Self::new(value.x() as f32, value.y() as f32)
    }
}

#[cfg(feature = "glam")]
impl From<glam::Vec2> for Vertex {
    fn from(value: glam::Vec2) -> Self {
        Self::new(value.x as f64, value.y as f64)
    }
}

#[cfg(feature = "glam")]
impl From<Vertex> for glam::DVec2 {
    fn from(value: Vertex) -> Self {
        Self::new(value.x(), value.y())
    }
}

#[cfg(feature = "glam")]
impl From<glam::DVec2> for Vertex {
    fn from(value: glam::DVec2) -> Self {
        Self::new(value.x, value.y)
    }
}

// impl PathC {
//     pub fn vertices(&self) -> &[VertexC] {
//         unsafe { std::slice::from_raw_parts(self.vertices, self.vertices_count) }
//     }
// }

// impl PartialEq for PathC {
//     fn eq(&self, other: &Self) -> bool {
//         self.closed == other.closed && self.vertices() == other.vertices()
//     }
// }

// impl Eq for PathC {}

// pub type Path = Vec<Vertex>;

// impl From<Path> for PathC {
//     fn from(value: Path) -> Self {
//         let mut vertices: Vec<[i64; 2]> = Vec::with_capacity(value.vertices.len());
//         for v in value.vertices.iter() {
//             vertices.push([v.0, v.1]);
//         }
//         let vertices_boxed = vertices.into_boxed_slice();
//         let vertices_ptr = Box::into_raw(vertices_boxed) as *mut [i64; 2];

//         Self {
//             vertices: vertices_ptr,
//             vertices_count: value.vertices.len(),
//             closed: if value.closed { 1 } else { 0 },
//         }
//     }
// }

// impl From<PathC> for Path {
//     fn from(value: PathC) -> Self {
//         let mut vertices: Vec<Vertex> = Vec::with_capacity(value.vertices_count);
//         for v in value.vertices().iter() {
//             vertices.push(Vertex(v[0], v[1]));
//         }

//         vertices
//     }
// }

// pub type Paths = Vec<Path>;

// impl PolygonC {
//     pub fn paths(&self) -> &[PathC] {
//         unsafe { std::slice::from_raw_parts(self.paths, self.paths_count) }
//     }
// }

// impl PartialEq for PolygonC {
//     fn eq(&self, other: &Self) -> bool {
//         self.type_ == other.type_ && self.paths() == other.paths()
//     }
// }

// impl Eq for PolygonC {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ClipType {
    #[default]
    None,
    Intersection,
    Union,
    Difference,
    Xor,
}

impl From<ClipTypeC> for ClipType {
    fn from(value: ClipTypeC) -> Self {
        match value {
            ClipTypeC_ctNone => ClipType::None,
            ClipTypeC_ctIntersection => ClipType::Intersection,
            ClipTypeC_ctUnion => ClipType::Union,
            ClipTypeC_ctDifference => ClipType::Difference,
            ClipTypeC_ctXor => ClipType::Xor,
            _ => unreachable!(),
        }
    }
}

impl From<ClipType> for ClipTypeC {
    fn from(value: ClipType) -> Self {
        match value {
            ClipType::None => ClipTypeC_ctNone,
            ClipType::Intersection => ClipTypeC_ctIntersection,
            ClipType::Union => ClipTypeC_ctUnion,
            ClipType::Difference => ClipTypeC_ctDifference,
            ClipType::Xor => ClipTypeC_ctXor,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum JoinType {
    #[default]
    Square,
    Bevel,
    Round,
    Miter,
}

impl From<JoinTypeC> for JoinType {
    fn from(value: JoinTypeC) -> Self {
        match value {
            JoinTypeC_jtSquare => JoinType::Square,
            JoinTypeC_jtBevel => JoinType::Bevel,
            JoinTypeC_jtRound => JoinType::Round,
            JoinTypeC_jtMiter => JoinType::Miter,
            _ => unreachable!(),
        }
    }
}

impl From<JoinType> for JoinTypeC {
    fn from(value: JoinType) -> Self {
        match value {
            JoinType::Square => JoinTypeC_jtSquare,
            JoinType::Bevel => JoinTypeC_jtBevel,
            JoinType::Round => JoinTypeC_jtRound,
            JoinType::Miter => JoinTypeC_jtMiter,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum EndType {
    #[default]
    ClosedPolygon,
    ClosedJoined,
    OpenButt,
    OpenSquare,
    OpenRound,
}

impl From<EndTypeC> for EndType {
    fn from(value: EndTypeC) -> Self {
        match value {
            EndTypeC_etClosedPolygon => EndType::ClosedPolygon,
            EndTypeC_etClosedJoined => EndType::ClosedJoined,
            EndTypeC_etOpenButt => EndType::OpenButt,
            EndTypeC_etOpenSquare => EndType::OpenSquare,
            EndTypeC_etOpenRound => EndType::OpenRound,
            _ => unreachable!(),
        }
    }
}

impl From<EndType> for EndTypeC {
    fn from(value: EndType) -> Self {
        match value {
            EndType::ClosedPolygon => EndTypeC_etClosedPolygon,
            EndType::ClosedJoined => EndTypeC_etClosedJoined,
            EndType::OpenButt => EndTypeC_etOpenButt,
            EndType::OpenSquare => EndTypeC_etOpenSquare,
            EndType::OpenRound => EndTypeC_etOpenRound,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PathType {
    #[default]
    Subject,
    Clip,
}

impl From<PathTypeC> for PathType {
    fn from(value: PathTypeC) -> Self {
        match value {
            PathTypeC_ptSubject => PathType::Subject,
            PathTypeC_ptClip => PathType::Clip,
            _ => unreachable!(),
        }
    }
}

impl From<PathType> for PathTypeC {
    fn from(value: PathType) -> Self {
        match value {
            PathType::Subject => PathTypeC_ptSubject,
            PathType::Clip => PathTypeC_ptClip,
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex() {
        assert_eq!(Vertex::from([1.0, 2.0]), Vertex(100, 200));
    }

    #[test]
    fn test_path() {
        let path = Path::new(vec![Vertex(1_000, 2_000), Vertex(3_000, 4_000)], true);

        assert_eq!(
            path.vertices(),
            &vec![Vertex(1_000, 2_000), Vertex(3_000, 4_000)],
        );
        assert_eq!(path.closed(), true);
    }

    #[test]
    fn test_polygon() {
        let polygon = Polygon::new(
            vec![Path::new(
                vec![Vertex(1_000, 2_000), Vertex(3_000, 4_000)],
                true,
            )],
            PathType::Subject,
        );

        assert_eq!(
            polygon.paths(),
            &vec![Path::new(
                vec![Vertex(1_000, 2_000), Vertex(3_000, 4_000)],
                true
            )],
        );
        assert_eq!(polygon.type_(), &PathType::Subject);
    }

    #[test]
    fn test_polygons() {
        let polygons = Polygons::new(vec![Polygon::new(
            vec![Path::new(
                vec![Vertex(1_000, 2_000), Vertex(3_000, 4_000)],
                true,
            )],
            PathType::Subject,
        )]);

        assert_eq!(
            polygons.polygons(),
            &vec![Polygon::new(
                vec![Path::new(
                    vec![Vertex(1_000, 2_000), Vertex(3_000, 4_000)],
                    true
                )],
                PathType::Subject
            )],
        );
    }

    #[test]
    fn test_join_type() {
        assert_eq!(JoinType::from(JoinTypeC_jtSquare), JoinType::Square);
        assert_eq!(JoinType::from(JoinTypeC_jtBevel), JoinType::Bevel);
        assert_eq!(JoinType::from(JoinTypeC_jtRound), JoinType::Round);
        assert_eq!(JoinType::from(JoinTypeC_jtMiter), JoinType::Miter);
    }

    #[test]
    fn test_end_type() {
        assert_eq!(
            EndType::from(EndTypeC_etClosedPolygon),
            EndType::ClosedPolygon
        );
        assert_eq!(
            EndType::from(EndTypeC_etClosedJoined),
            EndType::ClosedJoined
        );
        assert_eq!(EndType::from(EndTypeC_etOpenButt), EndType::OpenButt);
        assert_eq!(EndType::from(EndTypeC_etOpenSquare), EndType::OpenSquare);
        assert_eq!(EndType::from(EndTypeC_etOpenRound), EndType::OpenRound);
    }

    #[test]
    fn test_path_type() {
        assert_eq!(PathType::from(PathTypeC_ptSubject), PathType::Subject);
        assert_eq!(PathType::from(PathTypeC_ptClip), PathType::Clip);
    }

    #[test]
    fn test_clip_type() {
        assert_eq!(ClipType::from(ClipTypeC_ctNone), ClipType::None);
        assert_eq!(
            ClipType::from(ClipTypeC_ctIntersection),
            ClipType::Intersection
        );
        assert_eq!(ClipType::from(ClipTypeC_ctUnion), ClipType::Union);
        assert_eq!(ClipType::from(ClipTypeC_ctDifference), ClipType::Difference);
        assert_eq!(ClipType::from(ClipTypeC_ctXor), ClipType::Xor);
    }

    #[test]
    fn test_inflate() {
        let path = Path::new(
            vec![
                Vertex::new(0.0, 0.0),
                Vertex::new(10.0, 20.0),
                Vertex::new(20.0, 0.0),
            ],
            true,
        );
        let polygons = Polygons::new(vec![Polygon::new(vec![path.clone()], PathType::Subject)]);

        let output = inflate(
            polygons,
            3.0,
            JoinType::Square,
            EndType::ClosedPolygon,
            0.0,
            0.0,
        );

        assert_eq!(
            output
                .polygons()
                .first()
                .unwrap()
                .paths()
                .first()
                .unwrap()
                .vertices(),
            &vec![
                Vertex::new(-1.673, -3.0),
                Vertex::new(-3.431, -0.154),
                Vertex::new(8.15, 23.0),
                Vertex::new(11.854, 23.0),
                Vertex::new(23.431, -0.154),
                Vertex::new(21.673, -3.0),
            ]
        );
    }

    #[test]
    fn test_inflate_negative_delta() {
        let path = Path::new(
            vec![
                Vertex::new(0.0, 0.0),
                Vertex::new(10.0, 20.0),
                Vertex::new(20.0, 0.0),
            ],
            true,
        );
        let polygons = Polygons::new(vec![Polygon::new(vec![path.clone()], PathType::Subject)]);

        let output = inflate(
            polygons,
            -3.0,
            JoinType::Square,
            EndType::ClosedPolygon,
            0.0,
            0.0,
        );

        assert_eq!(
            output
                .polygons()
                .first()
                .unwrap()
                .paths()
                .first()
                .unwrap()
                .vertices(),
            &vec![
                Vertex::new(15.15, 3.0),
                Vertex::new(4.854, 3.0),
                Vertex::new(10.0, 13.3)
            ]
        );
    }

    #[test]
    fn test_intersect() {
        let path1 = Path::new(
            vec![
                Vertex::new(0.0, 0.0),
                Vertex::new(10.0, 0.0),
                Vertex::new(10.0, 10.0),
                Vertex::new(0.0, 10.0),
            ],
            true,
        );
        let path2 = Path::new(
            vec![
                Vertex::new(5.0, 5.0),
                Vertex::new(15.0, 5.0),
                Vertex::new(15.0, 15.0),
                Vertex::new(5.0, 15.0),
            ],
            true,
        );
        let subjects = Polygons::new(vec![Polygon::new(vec![path1.clone()], PathType::Subject)]);
        let clips = Polygons::new(vec![Polygon::new(vec![path2.clone()], PathType::Clip)]);

        let output = intersect(subjects, clips);

        assert_eq!(
            output
                .polygons()
                .first()
                .unwrap()
                .paths()
                .first()
                .unwrap()
                .vertices(),
            &vec![
                Vertex::new(10.0, 10.0),
                Vertex::new(5.0, 10.0),
                Vertex::new(5.0, 5.0),
                Vertex::new(10.0, 5.0),
            ]
        );
    }

    #[test]
    fn test_union() {
        let path1 = Path::new(
            vec![
                Vertex::new(0.0, 0.0),
                Vertex::new(10.0, 0.0),
                Vertex::new(10.0, 10.0),
                Vertex::new(0.0, 10.0),
            ],
            true,
        );
        let path2 = Path::new(
            vec![
                Vertex::new(5.0, 5.0),
                Vertex::new(15.0, 5.0),
                Vertex::new(15.0, 15.0),
                Vertex::new(5.0, 15.0),
            ],
            true,
        );
        let polygons = Polygons::new(vec![Polygon::new(
            vec![path1.clone(), path2.clone()],
            PathType::Subject,
        )]);

        let output = union(polygons);

        assert_eq!(
            output
                .polygons()
                .first()
                .unwrap()
                .paths()
                .first()
                .unwrap()
                .vertices(),
            &vec![
                Vertex::new(10.0, 5.0),
                Vertex::new(15.0, 5.0),
                Vertex::new(15.0, 15.0),
                Vertex::new(5.0, 15.0),
                Vertex::new(5.0, 10.0),
                Vertex::new(0.0, 10.0),
                Vertex::new(0.0, 0.0),
                Vertex::new(10.0, 0.0)
            ]
        );
    }

    #[test]
    fn test_difference() {
        let path1 = Path::new(
            vec![
                Vertex::new(0.0, 0.0),
                Vertex::new(10.0, 0.0),
                Vertex::new(10.0, 10.0),
                Vertex::new(0.0, 10.0),
            ],
            true,
        );
        let path2 = Path::new(
            vec![
                Vertex::new(5.0, 5.0),
                Vertex::new(15.0, 5.0),
                Vertex::new(15.0, 15.0),
                Vertex::new(5.0, 15.0),
            ],
            true,
        );
        let subjects = Polygons::new(vec![Polygon::new(vec![path1.clone()], PathType::Subject)]);
        let clips = Polygons::new(vec![Polygon::new(vec![path2.clone()], PathType::Clip)]);

        let output = difference(subjects, clips);

        assert_eq!(
            output
                .polygons()
                .first()
                .unwrap()
                .paths()
                .first()
                .unwrap()
                .vertices(),
            &vec![
                Vertex::new(10.0, 5.0),
                Vertex::new(5.0, 5.0),
                Vertex::new(5.0, 10.0),
                Vertex::new(0.0, 10.0),
                Vertex::new(0.0, 0.0),
                Vertex::new(10.0, 0.0),
            ]
        );
    }

    #[test]
    fn test_xor() {
        let path1 = Path::new(
            vec![
                Vertex::new(0.0, 0.0),
                Vertex::new(10.0, 0.0),
                Vertex::new(10.0, 10.0),
                Vertex::new(0.0, 10.0),
            ],
            true,
        );
        let path2 = Path::new(
            vec![
                Vertex::new(5.0, 5.0),
                Vertex::new(15.0, 5.0),
                Vertex::new(15.0, 15.0),
                Vertex::new(5.0, 15.0),
            ],
            true,
        );
        let subjects = Polygons::new(vec![Polygon::new(vec![path1.clone()], PathType::Subject)]);
        let clips = Polygons::new(vec![Polygon::new(vec![path2.clone()], PathType::Clip)]);

        let output = xor(subjects, clips);

        assert_eq!(
            output
                .polygons()
                .first()
                .unwrap()
                .paths()
                .first()
                .unwrap()
                .vertices(),
            &vec![
                Vertex::new(15.0, 15.0),
                Vertex::new(5.0, 15.0),
                Vertex::new(5.0, 10.0),
                Vertex::new(10.0, 10.0),
                Vertex::new(10.0, 5.0),
                Vertex::new(15.0, 5.0),
            ]
        );
    }
}
*/