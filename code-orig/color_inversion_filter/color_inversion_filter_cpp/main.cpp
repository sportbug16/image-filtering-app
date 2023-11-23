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

    // Invert the colors of the image
    cv::Mat inverted_img;
    cv::bitwise_not(img, inverted_img);

    // Save the inverted color image using OpenCV
    cv::imwrite("color_inverted.jpg", inverted_img);

    return 0;
}
