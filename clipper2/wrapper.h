#ifndef WRAPPER_H
#define WRAPPER_H

#include <stdint.h>
#include <stdlib.h>

// Make sure these includes are outside of the extern "C" for C++ compilation
#include "clipper.core.h"
#include "clipper.h"

#ifdef __cplusplus
// C++-specific struct definition including constructor
struct RustFriendlyPaths
{
    const Clipper2Lib::Point64 **paths;
    const size_t *path_lengths;
    size_t num_paths;

    // Ensure the order here matches the declaration order
    RustFriendlyPaths(size_t num_paths, const Clipper2Lib::Point64 **paths, const size_t *path_lengths);
};
#endif // __cplusplus

#ifdef __cplusplus
extern "C"
{
#endif

    struct RustFriendlyPaths;

    // Function declarations with C linkage
    RustFriendlyPaths *union_c(const RustFriendlyPaths &subjects, Clipper2Lib::FillRule fillrule);
    void free_rust_friendly_paths_c(RustFriendlyPaths *paths);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WRAPPER_H
