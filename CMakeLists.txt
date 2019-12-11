cmake_minimum_required(VERSION 3.13)
list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

# set the project name
project(phaeton VERSION 1.0)

# include osmium
find_package(Osmium REQUIRED COMPONENTS io)
include_directories(SYSTEM ${OSMIUM_INCLUDE_DIRS})

# add the executable
add_executable(phaeton src/phaeton.cpp)

include_directories(include)
