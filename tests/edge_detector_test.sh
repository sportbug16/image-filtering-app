#!/bin/bash

# Rust
echo "Running Edge Detection in Rust..."
(time ./edge_detection_rust) 2> rust_edge_detection_time.txt
(valgrind --leak-check=full --show-leak-kinds=all ./edge_detection_rust) 2> rust_edge_detection_memory.txt

# C++
echo "Running Edge Detection in C++..."
(time ./edge_detection_cpp) 2> cpp_edge_detection_time.txt
(valgrind --leak-check=full --show-leak-kinds=all ./edge_detection_cpp) 2> cpp_edge_detection_memory.txt
