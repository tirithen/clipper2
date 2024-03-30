#include "wrapper.h"

#ifdef __cplusplus
extern "C"
{
#endif

    Clipper2Lib::Paths64 convert_to_paths64(const PathsC *paths);
    PathsC *convert_from_paths64(const Clipper2Lib::Paths64 &paths);

    PathsC *union_c(const PathsC *subjects, FillRuleC fillrule)
    {
        Clipper2Lib::Paths64 subjects64 = convert_to_paths64(subjects);
        Clipper2Lib::Paths64 result = Clipper2Lib::Union(subjects64, static_cast<Clipper2Lib::FillRule>(fillrule));
        return convert_from_paths64(result);
    }

    void free_paths_c(PathsC *paths)
    {
        delete paths;
    }

    const Point *get_points(const PathsC *paths)
    {
        return paths->points.data();
    }

    const size_t *get_path_starts(const PathsC *paths)
    {
        return paths->path_starts.data();
    }

    size_t get_num_paths(const PathsC *paths)
    {
        return paths->num_paths;
    }
#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

Clipper2Lib::Paths64 convert_to_paths64(const PathsC *paths)
{
    Clipper2Lib::Paths64 result;
    result.reserve(paths->num_paths);
    size_t current_index = 0;

    for (size_t i = 0; i < paths->num_paths; ++i)
    {
        result.push_back(Clipper2Lib::Path64());
        for (size_t j = 0; j < paths->path_starts[i]; ++j)
        {
            const auto &p = paths->points[current_index + j];
            result.back().emplace_back(p.x, p.y);
        }
        current_index += paths->path_starts[i];
    }
    return result;
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

#endif
