cmake_minimum_required(VERSION 3.13)
list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

# set the project name
project(phaeton VERSION 1.0)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# specify the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# include osmium
find_package(Osmium REQUIRED COMPONENTS io pbf)
include_directories(SYSTEM ${OSMIUM_INCLUDE_DIRS})
include_directories(${OSMIUM_INCLUDE_DIRS})


# add the executable
add_executable(phaeton src/phaeton.cpp)

# add links
target_link_libraries(phaeton -lpthread -lz -lexpat -lbz2)
