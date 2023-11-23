extern crate image;

use image::{GenericImageView, Rgba, RgbaImage};
use imageproc::edges::sobel;

fn main() {
    // Load an image using the image crate
    let img = image::open("input.jpg").expect("Failed to open image");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Create a new RgbaImage with the same dimensions
    let mut output_image = RgbaImage::new(width, height);

    // Apply the Sobel edge detection algorithm
    sobel(&img.to_rgba(), &mut output_image);

    // Save the edge-detected image
    output_image.save("edge_detected.jpg").expect("Failed to save image");
}
