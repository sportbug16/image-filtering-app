extern crate image;
use image::{GenericImageView, Rgba, RgbaImage};
use imageproc::filter::blur;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Load an image using the image crate
    let img = image::open("input.jpg").expect("Failed to open image");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Create a new RgbaImage with the same dimensions
    let mut output_image = RgbaImage::new(width, height);

    // Apply a Gaussian blur to the image using multiple threads
    let img_arc = Arc::new(img.to_rgba());
    let output_image_arc = Arc::new(Mutex::new(&mut output_image));

    let handles: Vec<_> = (0..4).map(|_| {
        let img_clone = Arc::clone(&img_arc);
        let output_clone = Arc::clone(&output_image_arc);

        thread::spawn(move || {
            blur(&img_clone, &mut *output_clone.lock().unwrap(), 2.0);
        })
    }).collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Save the blurred image
    output_image.save("blurred_rust.jpg").expect("Failed to save image");
}
