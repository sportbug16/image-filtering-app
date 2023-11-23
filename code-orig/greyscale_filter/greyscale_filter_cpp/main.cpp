#include <opencv2/opencv.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/imgproc/imgproc.hpp>

int main() {
    
    cv::Mat img = cv::imread("input.jpg");

    if (img.empty()) {
        std::cerr << "Failed to open image" << std::endl;
        return -1;
    }

    
    cv::Mat grayscale_img;
    cv::cvtColor(img, grayscale_img, cv::COLOR_BGR2GRAY);

    
    cv::imwrite("output.jpg", grayscale_img);

    return 0;
}


