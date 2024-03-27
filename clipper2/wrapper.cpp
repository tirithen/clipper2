#include "wrapper.h"
#include "clipper.core.h"
#include "clipper.h"

#ifdef __cplusplus

RustFriendlyPathsC::RustFriendlyPathsC(size_t num_paths, const PointC **paths, const size_t *path_lengths)
    : num_paths(num_paths), paths(paths), path_lengths(path_lengths) {}

Clipper2Lib::Paths64 convert_to_paths64(const RustFriendlyPathsC *subjects)
{
    Clipper2Lib::Paths64 paths;
    for (size_t i = 0; i < subjects->num_paths; ++i)
    {
        paths.emplace_back(subjects->paths[i], subjects->paths[i] + subjects->path_lengths[i]);
    }
    return paths;
}

RustFriendlyPathsC *convert_from_paths64(const Clipper2Lib::Paths64 &paths)
{
    size_t size = paths.size();
    auto **tempPaths = new const PointC *[size];
    auto *tempPathLengths = new size_t[size];

    for (size_t i = 0; i < size; ++i)
    {
        tempPathLengths[i] = paths[i].size();
        // tempPaths[i] = const_cast<PointC *>(paths[i].data());
        tempPaths[i] = reinterpret_cast<const PointC *>(&paths[i]);
    }

    auto *rustFriendlyPathsC = new RustFriendlyPathsC(size, tempPaths, tempPathLengths);
    return rustFriendlyPathsC;
}

#endif

extern "C"
{
    RustFriendlyPathsC *union_c(const RustFriendlyPathsC *subjects, FillRuleC fillrule)
    {
        Clipper2Lib::FillRule cppFillRule = static_cast<Clipper2Lib::FillRule>(fillrule);
        Clipper2Lib::Paths64 subjects_cpp = convert_to_paths64(subjects);
        Clipper2Lib::Paths64 result_cpp = Clipper2Lib::Union(subjects_cpp, cppFillRule);
        return convert_from_paths64(result_cpp);
    }

    RustFriendlyPathsC *create_rust_friendly_paths_c(size_t num_paths, const PointC **paths, const size_t *path_lengths)
    {
        return new RustFriendlyPathsC(num_paths, paths, path_lengths);
    }

    const PointC **get_rust_paths_ptr_c(const RustFriendlyPathsC *rustFriendlyPathsC)
    {
        return rustFriendlyPathsC->paths;
    }

    const size_t *get_rust_path_lengths_ptr_c(const RustFriendlyPathsC *rustFriendlyPathsC)
    {
        return rustFriendlyPathsC->path_lengths;
    }

    size_t get_rust_num_paths_c(const RustFriendlyPathsC *rustFriendlyPathsC)
    {
        return rustFriendlyPathsC->num_paths;
    }

    void free_rust_friendly_paths_c(RustFriendlyPathsC *paths)
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