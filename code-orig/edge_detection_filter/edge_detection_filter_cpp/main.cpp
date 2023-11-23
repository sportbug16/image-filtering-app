#include <opencv2/opencv.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/imgproc/imgproc.hpp>

int main()
{
    // Load an image using OpenCV
    cv::Mat img = cv::imread("input.jpg");

    if (img.empty())
    {
        std::cerr << "Failed to open image" << std::endl;
        return -1;
    }

    // Convert the image to grayscale
    cv::Mat grayscale_img;
    cv::cvtColor(img, grayscale_img, cv::COLOR_BGR2GRAY);

    // Apply the Canny edge detection algorithm
    cv::Mat edges;
    cv::Canny(grayscale_img, edges, 50, 150);

    // Save the edge-detected image using OpenCV
    cv::imwrite("edge_detected.jpg", edges);

    return 0;
}
