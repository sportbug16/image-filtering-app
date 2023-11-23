#!/bin/bash

# Rust
echo "Running Grayscale Filter in Rust..."
(time ./grayscale_rust) 2> rust_grayscale_time.txt
(valgrind --leak-check=full --show-leak-kinds=all ./grayscale_rust) 2> rust_grayscale_memory.txt

# C++
echo "Running Grayscale Filter in C++..."
(time ./grayscale_cpp) 2> cpp_grayscale_time.txt
(valgrind --leak-check=full --show-leak-kinds=all ./grayscale_cpp) 2> cpp_grayscale_memory.txt
