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
    } PointC;

    typedef enum
    {
        EvenOdd,
        NonZero,
        Positive,
        Negative
    } FillRuleC;

    // Forward declaration. No need for "struct" tag here in typedef.
    typedef struct PathsC PathsC;

    PathsC *union_c(const PointC *points, size_t num_paths, const size_t *path_sizes, FillRuleC fillrule);
    void free_paths_c(PathsC *paths);

#ifdef __cplusplus
} // extern "C"
#endif

#ifdef __cplusplus
#include <vector>
#include "clipper.h"

// Define PathsC struct in the C++ section only
struct PathsC
{
    std::vector<PointC> points;      // Flattened vector of points
    std::vector<size_t> path_starts; // Starting index of each path in points vector
    size_t num_paths;
};

#endif // __cplusplus

#endif // WRAPPER_H
