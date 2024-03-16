#include "wrapper.h"
#include "clipper.core.h"
#include "clipper.h"

using Clipper2Lib::InflatePaths;
using Clipper2Lib::Paths64;
using Clipper2Lib::Point64;
using Clipper2Lib::PolyTree64;

Clipper2Lib::Path64 get_path(const PathC &path)
{
    Clipper2Lib::Path64 clipper_path;
    auto path_size = path.vertices_count;
    clipper_path.reserve(path_size);

    for (size_t i = 0; i < path_size; ++i)
    {
        clipper_path.push_back(Point64(path.vertices[i][0], path.vertices[i][1]));
    }
    return clipper_path;
}

std::pair<Paths64, std::vector<bool>> get_polygon_paths(const PolygonC &polygon)
{
    Paths64 paths;
    paths.reserve(polygon.paths_count);

    std::vector<bool> closed;
    closed.reserve(polygon.paths_count);

    for (size_t i = 0; i < polygon.paths_count; ++i)
    {
        paths.push_back(get_path(polygon.paths[i]));
        closed.push_back(polygon.paths[i].closed);
    }
    return std::make_pair(paths, closed);
}

Clipper2Lib::Paths64 get_closed_paths_from_polygons(PolygonsC polygons)
{
    Paths64 paths;

    for (size_t i = 0; i < polygons.polygons_count; ++i)
    {
        auto &polygon = polygons.polygons[i];
        auto paths_closed = get_polygon_paths(polygon);
        Paths64 &next_paths = paths_closed.first;

        paths.reserve(paths.size() + next_paths.size());
        paths.insert(paths.end(), next_paths.begin(), next_paths.end());
    }

    return paths;
}

PathC get_path_from_closed_path(Clipper2Lib::Path64 &clipper_path)
{
    PathC path;
    path.vertices_count = clipper_path.size();
    path.vertices = new VertexC[path.vertices_count];
    path.closed = true;
    for (size_t i = 0; i < path.vertices_count; ++i)
    {
        path.vertices[i][0] = clipper_path[i].x;
        path.vertices[i][1] = clipper_path[i].y;
    }
    return path;
}

PolygonsC get_polygons_from_closed_paths(Clipper2Lib::Paths64 &closed_paths)
{
    std::vector<PolygonC> polygon_vector;

    for (size_t i = 0; i < closed_paths.size(); ++i)
    {
        PolygonC polygon;
        polygon.type = ptSubject;
        polygon.paths_count = 1;
        polygon.paths = new PathC[1];
        polygon.paths[0] = get_path_from_closed_path(closed_paths[i]);
        polygon_vector.push_back(polygon);
    }

    PolygonsC polygons;
    polygons.polygons_count = polygon_vector.size();
    polygons.polygons = new PolygonC[polygons.polygons_count];
    std::copy(polygon_vector.begin(), polygon_vector.end(), polygons.polygons);

    return polygons;
}

Clipper2Lib::JoinType to_cpp_join_type(JoinTypeC from)
{
    switch (from)
    {
    case JoinTypeC::jtSquare:
        return Clipper2Lib::JoinType::Square;
    case JoinTypeC::jtBevel:
        return Clipper2Lib::JoinType::Bevel;
    case JoinTypeC::jtRound:
        return Clipper2Lib::JoinType::Round;
    case JoinTypeC::jtMiter:
        return Clipper2Lib::JoinType::Miter;
    default:
        throw std::invalid_argument("Unknown JoinTypeC value");
    }
}

Clipper2Lib::EndType to_cpp_end_type(EndTypeC from)
{
    switch (from)
    {
    case EndTypeC::etClosedPolygon:
        return Clipper2Lib::EndType::Polygon;
    case EndTypeC::etClosedJoined:
        return Clipper2Lib::EndType::Joined;
    case EndTypeC::etOpenButt:
        return Clipper2Lib::EndType::Butt;
    case EndTypeC::etOpenSquare:
        return Clipper2Lib::EndType::Square;
    case EndTypeC::etOpenRound:
        return Clipper2Lib::EndType::Round;
    default:
        throw std::invalid_argument("Unknown EndTypeC value");
    }
}

PolygonsC inflate_c(
    PolygonsC polygons,
    double delta,
    JoinTypeC join_type = jtMiter,
    EndTypeC end_type = etClosedPolygon,
    double miter_limit = 2.0,
    double arc_tolerance = 0.0)
{
    Paths64 polygons_paths = get_closed_paths_from_polygons(polygons);
    Paths64 paths = InflatePaths(
        polygons_paths,
        delta,
        to_cpp_join_type(join_type),
        to_cpp_end_type(end_type),
        miter_limit,
        arc_tolerance);
    return get_polygons_from_closed_paths(paths);
}

PolygonsC intersect_c(PolygonsC subjects, PolygonsC clips) {
    Paths64 subjects_paths = get_closed_paths_from_polygons(subjects);
    Paths64 clips_paths = get_closed_paths_from_polygons(clips);
    Paths64 result = Clipper2Lib::Intersect(subjects_paths, clips_paths, Clipper2Lib::FillRule::NonZero);
    return get_polygons_from_closed_paths(result);
}

PolygonsC union_c(PolygonsC subjects) {
    Paths64 subjects_paths = get_closed_paths_from_polygons(subjects);
    Paths64 result = Clipper2Lib::Union(subjects_paths, Clipper2Lib::FillRule::NonZero);
    return get_polygons_from_closed_paths(result);
}

PolygonsC difference_c(PolygonsC subjects, PolygonsC clips) {
    Paths64 subjects_paths = get_closed_paths_from_polygons(subjects);
    Paths64 clips_paths = get_closed_paths_from_polygons(clips);
    Paths64 result = Clipper2Lib::Difference(subjects_paths, clips_paths, Clipper2Lib::FillRule::NonZero);
    return get_polygons_from_closed_paths(result);
}

PolygonsC xor_c(PolygonsC subjects, PolygonsC clips) {
    Paths64 subjects_paths = get_closed_paths_from_polygons(subjects);
    Paths64 clips_paths = get_closed_paths_from_polygons(clips);
    Paths64 result = Clipper2Lib::Xor(subjects_paths, clips_paths, Clipper2Lib::FillRule::NonZero);
    return get_polygons_from_closed_paths(result);
}

void free_path_c(PathC path)
{
    delete[] path.vertices;
}

void free_polygon_c(PolygonC polygon)
{
    for (size_t i = 0; i < polygon.paths_count; ++i)
    {
        free_path_c(polygon.paths[i]);
    }
    delete[] polygon.paths;
}

void free_polygons_c(PolygonsC polygons)
{
    for (size_t i = 0; i < polygons.polygons_count; ++i)
    {
        free_polygon_c(polygons.polygons[i]);
    }
    delete[] polygons.polygons;
}
