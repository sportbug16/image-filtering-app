#include <opencv2/opencv.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/imgproc/imgproc.hpp>
#include <thread>
#include <mutex>

int main()
{
    // Load an image using OpenCV
    cv::Mat img = cv::imread("input.jpg");

    if (img.empty())
    {
        std::cerr << "Failed to open image" << std::endl;
        return -1;
    }

    // Apply Gaussian blur to the image using multiple threads
    cv::Mat blurred_img = img.clone();

    std::vector<std::thread> threads;
    std::mutex mutex;

    for (int i = 0; i < 4; ++i)
    {
        threads.emplace_back([&img, &blurred_img, &mutex]()
                             { cv::GaussianBlur(img, blurred_img, cv::Size(0, 0), 2.0); });
    }

    // Wait for all threads to finish
    for (auto &thread : threads)
    {
        thread.join();
    }

    // Save the blurred image using OpenCV
    cv::imwrite("blurred_cpp.jpg", blurred_img);

    return 0;
}
