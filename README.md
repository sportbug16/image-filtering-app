# image-filtering-app

## Problem Statement:

The project addresses the task of image processing and filtering, specifically applying grayscale filters among others like blur filter, edge detection, color inversion etc to images among other things. These filters are a common operation used in various image processing applications. Image processing and filtering tools are often made in low level languages for fast deployment at large scale. 

**What are we solving?**
We are using rust to implement these features and currently in many other applications at low level languages c++ is used to show better memory management and get better speeds. This problem hasn't been solved before exactly in our way. We will show that Rust implementation is better in terms of performance and ease of use with its cargo library.

## Software Architecture:
We haven't reused any existing code and have taken help of library documentations of OpenCV for c++ and "image" crate for Rust. The architecture is monolithic and doesn't involve any client server as such and we have run tests on a single computer device and done benchmarking. The testing component is remote.

## POPL Aspects:
**1)Better error handling in Rust over C++:** 
Rust: Uses expect for error handling, which panics with a custom error message if the image fails to open. ![image](https://github.com/sportbug16/image-filtering-app/assets/70038936/abf30726-3a2e-439d-9c49-249fd6b81602)

C++: Uses traditional error-checking with if (img.empty()) and printing an error message.
![image](https://github.com/sportbug16/image-filtering-app/assets/70038936/c4ba4f38-2172-40ad-8aef-53243f75e682)

**2)Variable immutability enabled consistency:**
Rust: Variables are immutable by default. The let mut output_image explicitly declares output_image as mutable.
C++: Uses cv::Mat for image representation. Variables are mutable by default.

**3)Strong Type language:**
Rust: Allows explicit type conversion (as) for numeric operations.
C++: Requires explicit casting for type conversions.

**4)Memory Safety:**
Rust: Guarantees memory safety through ownership, borrowing, and lifetimes, reducing the risk of common programming errors like null pointer dereferencing.
C++: Manual memory management can lead to memory-related bugs, such as memory leaks or undefined behavior.

**5)Package Ecosystem enabled faster running and easier use:**
Rust: Benefits from the Cargo package manager, which simplifies dependency management and provides a centralized repository for packages.
![image](https://github.com/sportbug16/image-filtering-app/assets/70038936/3ce32691-9983-4b54-939c-d4a42fe57473)

C++: Dependency management often involves manual download and configuration of libraries, which can be more error-prone.![image](https://github.com/sportbug16/image-filtering-app/assets/70038936/f2e3a224-4eb1-4bc3-afb0-3279060603b6)


**6)Concurrency with Ownership:**
Rust: Uses ownership and Arc (atomic reference counting) to safely share data (img_arc and output_image_arc) among multiple threads. ![image](https://github.com/sportbug16/image-filtering-app/assets/70038936/c8ac8084-bbc0-4841-ac6c-d23eef495d47)

C++: Manually clones img and shares the data among threads. Ownership semantics are less explicit and rely on developers' careful management.

**8)Parallelization :**
Rust: Leverages Rust's ownership system to safely parallelize the blur operation across multiple threads.
C++: Uses threads for parallelization but requires manual management of shared resources and synchronization.

## Results:

We have used following publically available datasets to get to our results.

**Lenna Image:**
The Lenna image is a standard test image widely used in the field of image processing and computer vision. It is a photograph of Lena Söderberg, a Swedish model, taken by photographer Dwight Hooker. The image has become a standard reference in the development and evaluation of image processing algorithms.
The Lenna image is widely available online and can be accessed from various sources, including academic databases and websites dedicated to image processing

**Kodak Image Dataset:**
The Kodak Image Dataset is a collection of high-quality images provided by the Eastman Kodak Company. This dataset is commonly used for testing and evaluating image processing and compression algorithms.
The Kodak Image Dataset is available for download from various sources, including the official Kodak website and other image processing databases.

We used Rust's inbuilt Cargo and its '--release' flag to get memory usage and time usage.
For C++ we used "valgrind" library (specifically "memcheck") tool to visualize C++ memory usage.
![image](https://github.com/sportbug16/image-filtering-app/assets/70038936/1a7f08a8-4a06-412e-b481-14de2880c9f3)

