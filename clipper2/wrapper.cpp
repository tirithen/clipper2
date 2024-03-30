#include "wrapper.h"

#ifdef __cplusplus
extern "C"
{
#endif

    // Forward declaration of conversion functions
    Clipper2Lib::Paths64 convert_to_paths64(const PointC *points, size_t num_paths, const size_t *path_sizes);
    PathsC *convert_from_paths64(const Clipper2Lib::Paths64 &paths);

    PathsC *union_c(const PointC *points, size_t num_paths, const size_t *path_sizes, FillRuleC fillrule)
    {
        // Convert from C to C++ types
        Clipper2Lib::Paths64 subjects = convert_to_paths64(points, num_paths, path_sizes);

        // Perform the union operation
        Clipper2Lib::Paths64 result = Clipper2Lib::Union(subjects, static_cast<Clipper2Lib::FillRule>(fillrule));

        // Convert back to C types
        return convert_from_paths64(result);
    }

    void free_paths_c(PathsC *paths)
    {
        delete paths;
    }

#ifdef __cplusplus
} // extern "C"
#endif

#ifdef __cplusplus

Clipper2Lib::Paths64 convert_to_paths64(const PointC *points, size_t num_paths, const size_t *path_sizes)
{
    Clipper2Lib::Paths64 paths;
    paths.reserve(num_paths);
    size_t current_index = 0;

    for (size_t i = 0; i < num_paths; ++i)
    {
        paths.push_back(Clipper2Lib::Path64());
        for (size_t j = 0; j < path_sizes[i]; ++j)
        {
            const auto &p = points[current_index + j];
            paths.back().emplace_back(p.x, p.y);
        }
        current_index += path_sizes[i];
    }
    return paths;
}

PathsC *convert_from_paths64(const Clipper2Lib::Paths64 &paths)
{
    auto *result = new PathsC;
    result->num_paths = paths.size();
    result->path_starts.resize(paths.size());

    size_t total_points = 0;
    for (const auto &path : paths)
    {
        result->path_starts.push_back(total_points);
        for (const auto &point : path)
        {
            result->points.push_back({point.x, point.y});
            ++total_points;
        }
    }

    return result;
}

#endif // __cplusplus
