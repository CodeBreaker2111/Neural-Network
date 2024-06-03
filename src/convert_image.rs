use image::io::Reader as ImageReader;
use image::Luma;
use std::io::{self, Write};

pub fn image() -> Vec<f32> {
    // Prompt the user for the image path
    println!("Path to image: ");
    let mut image_path = String::new();
    io::stdin().read_line(&mut image_path).expect("Failed to read line");
    let image_path = image_path.trim(); // Remove any surrounding whitespace

    // Open the image
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();

    // Convert the image to grayscale
    let grayscale_img = img.to_luma8();

    // Convert the grayscale image to a flat vector of 1.0s and 0.0s
    let binary_representation = convert_to_binary(&grayscale_img);

    return binary_representation;
}

fn convert_to_binary(img: &image::GrayImage) -> Vec<f32> {
    let mut binary_representation = Vec::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y).0[0];
            if pixel > 128 {
                binary_representation.push(1.0); // White
            } else {
                binary_representation.push(0.0); // Black
            }
        }
    }
    binary_representation
}
