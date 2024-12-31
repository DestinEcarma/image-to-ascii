pub mod error;

mod ascii;
mod defs;

use defs::Result;
use image::GenericImageView as _;
use std::path::Path;

/// Convert an image to ASCII art.
pub fn convert(image: &Path, dimensions: (Option<u32>, Option<u32>)) -> Result<String> {
    // Load the image from the input path.
    let mut image = image::open(image)?;

    // Get the dimensions of the image.
    let (mut width, mut height) = image.dimensions();

    // Resize the image if the dimensions are specified.
    match dimensions {
        (Some(w), Some(h)) => {
            image = image.resize(w, h, image::imageops::FilterType::Nearest);
            width = w;
            height = h;
        }
        (Some(w), None) => {
            let ratio = w as f32 / width as f32;
            let h = (height as f32 * ratio).round() as u32;

            image = image.resize(w, h, image::imageops::FilterType::Nearest);
            width = w;
            height = h;
        }
        (None, Some(h)) => {
            let ratio = h as f32 / height as f32;
            let w = (width as f32 * ratio).round() as u32;

            image = image.resize(w, h, image::imageops::FilterType::Nearest);
            width = w;
            height = h;
        }
        _ => (),
    }

    let image = image.to_luma_alpha8();

    // Create a vector to store the raw ASCII art.
    let mut ascii_art = String::new();

    // Iterate over the pixels in the image and convert them to ASCII characters.
    for y in 0..height {
        for x in 0..width {
            let luma = image.get_pixel(x, y);

            ascii_art.push(ascii::to_ascii(luma));
        }

        // Add a newline character at the end of each row.
        ascii_art.push('\n');
    }

    Ok(ascii_art)
}
