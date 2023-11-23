extern crate image;

use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;

fn main() {
    
    let img = image::open("input.jpg").expect("Failed to open image");

    
    let (width, height) = img.dimensions();

    // Create a new ImageBuffer with the same dimensions and grayscale pixel format
    let mut output_image = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            
            let pixel = img.get_pixel(x, y);
            // Convert the pixel to grayscale
            let grayscale = (pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114) as u8;
            // Create a grayscale Rgba pixel
            let grayscale_pixel = Rgba([grayscale, grayscale, grayscale, pixel[3]]);
            
            output_image.put_pixel(x, y, grayscale_pixel);
        }
    }

    // Save the grayscale image
    output_image.save("output.jpg").expect("Failed to save image");
}

