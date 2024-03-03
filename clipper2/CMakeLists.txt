cmake_minimum_required(VERSION 3.15)
project(Clipper2 VERSION 1.3.0 LANGUAGES C CXX)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
if(NOT DEFINED CMAKE_CXX_STANDARD OR CMAKE_CXX_STANDARD LESS 17)
    set(CMAKE_CXX_STANDARD 17)
endif()
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

project(clipper2)

add_library(clipper2 STATIC clipper.engine.cpp clipper.offset.cpp clipper.rectclip.cpp wrapper.cpp)

install(TARGETS clipper2 DESTINATION .)
