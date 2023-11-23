#!/bin/bash

# Rust
echo "Running Blur Filter in Rust..."
(time ./blur_rust) 2> rust_blur_time.txt
(valgrind --leak-check=full --show-leak-kinds=all ./blur_rust) 2> rust_blur_memory.txt

# C++
echo "Running Blur Filter in C++..."
(time ./blur_cpp) 2> cpp_blur_time.txt
(valgrind --leak-check=full --show-leak-kinds=all ./blur_cpp) 2> cpp_blur_memory.txt
