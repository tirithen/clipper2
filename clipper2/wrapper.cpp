#include "wrapper.h"
#include "clipper.core.h"
#include "clipper.h"

#ifdef __cplusplus
// Correct definition of the constructor outside of the header file
RustFriendlyPaths::RustFriendlyPaths(size_t num_paths, const Clipper2Lib::Point64 **paths, const size_t *path_lengths)
    : num_paths(num_paths), paths(paths), path_lengths(path_lengths) {}
#endif

// Function to convert from RustFriendlyPaths to Paths64
Clipper2Lib::Paths64 convert_to_paths64(const RustFriendlyPaths &subjects)
{
    Clipper2Lib::Paths64 paths;
    for (size_t i = 0; i < subjects.num_paths; ++i)
    {
        paths.emplace_back(subjects.paths[i], subjects.paths[i] + subjects.path_lengths[i]);
    }
    return paths;
}

// Function to convert from Paths64 to RustFriendlyPaths
RustFriendlyPaths *convert_from_paths64(const Clipper2Lib::Paths64 &paths)
{
    size_t size = paths.size();
    auto **tempPaths = new const Clipper2Lib::Point64 *[size];
    auto *tempPathLengths = new size_t[size];

    for (size_t i = 0; i < size; ++i)
    {
        tempPathLengths[i] = paths[i].size();
        tempPaths[i] = const_cast<Clipper2Lib::Point64 *>(paths[i].data());
    }

    auto *rustFriendlyPaths = new RustFriendlyPaths(size, tempPaths, tempPathLengths);
    return rustFriendlyPaths;
}

extern "C"
{
    RustFriendlyPaths *union_c(const RustFriendlyPaths &subjects, Clipper2Lib::FillRule fillrule)
    {
        Clipper2Lib::Paths64 subjects_cpp = convert_to_paths64(subjects);
        Clipper2Lib::Paths64 result_cpp = Clipper2Lib::Union(subjects_cpp, fillrule);
        return convert_from_paths64(result_cpp);
    }

    void free_rust_friendly_paths_c(RustFriendlyPaths *paths)
    {
        for (size_t i = 0; i < paths->num_paths; ++i)
        {
            delete[] paths->paths[i];
        }
        delete[] paths->paths;
        delete[] paths->path_lengths;
        delete paths;
    }
}