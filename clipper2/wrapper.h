#ifndef WRAPPER_H
#define WRAPPER_H

#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct PointC
    {
        int64_t x;
        int64_t y;
    } PointC;

    typedef enum FillRuleC
    {
        EvenOdd,
        NonZero,
        Positive,
        Negative
    } FillRuleC;

    typedef struct RustFriendlyPathsC RustFriendlyPathsC;

    RustFriendlyPathsC *union_c(const RustFriendlyPathsC *subjects, FillRuleC fillrule);
    RustFriendlyPathsC *create_rust_friendly_paths_c(size_t num_paths, const PointC **paths, const size_t *path_lengths);
    const PointC **get_rust_paths_ptr_c(const RustFriendlyPathsC *rustFriendlyPathsC);
    const size_t *get_rust_path_lengths_ptr_c(const RustFriendlyPathsC *rustFriendlyPathsC);
    size_t get_rust_num_paths_c(const RustFriendlyPathsC *rustFriendlyPathsC);
    void free_rust_friendly_paths_c(RustFriendlyPathsC *paths);

#ifdef __cplusplus
}

#include "clipper.core.h"

struct RustFriendlyPathsC
{
    size_t num_paths;
    const PointC **paths;
    const size_t *path_lengths;

    RustFriendlyPathsC(size_t num_paths, const PointC **paths, const size_t *path_lengths);
};

#endif

#endif
