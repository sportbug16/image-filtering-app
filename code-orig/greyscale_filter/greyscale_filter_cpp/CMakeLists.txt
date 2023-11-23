cmake_minimum_required(VERSION 3.0)
project(GreyscaleFilter)

find_package(OpenCV REQUIRED)

add_executable(greyscale_filter main.cpp)
target_link_libraries(greyscale_filter ${OpenCV_LIBS})

