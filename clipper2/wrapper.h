#ifndef WRAPPER_H
#define WRAPPER_H

#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct
    {
        int64_t x, y;
    } Point;

    typedef enum
    {
        EvenOdd,
        NonZero,
        Positive,
        Negative
    } FillRuleC;

    typedef struct PathsC PathsC;

    PathsC *union_c(const PathsC *subjects, FillRuleC fillrule);
    void free_paths_c(PathsC *paths);

    const Point *get_points(const PathsC *paths);
    const size_t *get_path_starts(const PathsC *paths);
    size_t get_num_paths(const PathsC *paths);

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
#include <vector>
#include "clipper.h"

struct PathsC
{
    std::vector<Point> points;
    std::vector<size_t> path_starts;
    size_t num_paths;
};

#endif

#endif
