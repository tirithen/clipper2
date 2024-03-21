#ifndef WRAPPER_H
#define WRAPPER_H

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdint.h>
#include <stdlib.h>

    typedef enum ClipTypeC
    {
        ctNone,
        ctIntersection,
        ctUnion,
        ctDifference,
        ctXor
    } ClipTypeC;

    typedef enum JoinTypeC
    {
        jtSquare,
        jtBevel,
        jtRound,
        jtMiter
    } JoinTypeC;

    typedef enum EndTypeC
    {
        etClosedPolygon,
        etClosedJoined,
        etOpenButt,
        etOpenSquare,
        etOpenRound
    } EndTypeC;

    typedef enum PathTypeC
    {
        ptSubject,
        ptClip
    } PathTypeC;

    typedef int64_t VertexC[2];

    typedef struct PathC
    {
        VertexC *vertices;
        size_t vertices_count;
        int closed;
    } PathC;

    typedef struct PolygonC
    {
        PathC *paths;
        size_t paths_count;
        PathTypeC type;
    } PolygonC;

    typedef struct PolygonsC
    {
        PolygonC *polygons;
        size_t polygons_count;
    } PolygonsC;

    PolygonsC inflate_c(
        PolygonsC polygons,
        double delta,
        JoinTypeC join_type,
        EndTypeC end_type,
        double miter_limit,
        double arc_tolerance);

    PolygonsC intersect_c(PolygonsC subjects, PolygonsC clips);

    PolygonsC union_c(PolygonsC subjects);

    PolygonsC difference_c(PolygonsC subjects, PolygonsC clips);

    PolygonsC xor_c(PolygonsC subjects, PolygonsC clips);

    void free_path_c(PathC path);
    void free_polygon_c(PolygonC polygon);
    void free_polygons_c(PolygonsC polygons);

#ifdef __cplusplus
}
#endif

#endif // WRAPPER_H
