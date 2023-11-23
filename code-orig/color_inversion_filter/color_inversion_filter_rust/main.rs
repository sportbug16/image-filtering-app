extern crate image;

use image::{GenericImageView, Rgba, RgbaImage};

fn main() {
    // Load an image using the image crate
    let img = image::open("input.jpg").expect("Failed to open image");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Create a new RgbaImage with the same dimensions
    let mut output_image = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            // Get the pixel at (x, y)
            let pixel = img.get_pixel(x, y);
            // Invert the color of the pixel
            let inverted_pixel = Rgba([255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]]);
            // Put the inverted pixel in the output image
            output_image.put_pixel(x, y, inverted_pixel);
        }
    }

    // Save the inverted color image
    output_image.save("color_inverted.jpg").expect("Failed to save image");
}
