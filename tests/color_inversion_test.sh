#!/bin/bash

# Rust
echo "Running Color Inversion in Rust..."
(time ./color_inversion_rust) 2> rust_color_inversion_time.txt
(valgrind --leak-check=full --show-leak-kinds=all ./color_inversion_rust) 2> rust_color_inversion_memory.txt

# C++
echo "Running Color Inversion in C++..."
(time ./color_inversion_cpp) 2> cpp_color_inversion_time.txt
(valgrind --leak-check=full --show-leak-kinds=all ./color_inversion_cpp) 2> cpp_color_inversion_memory.txt
