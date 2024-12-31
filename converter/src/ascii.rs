use image::Pixel;

/// The ASCII character set used to represent intensity values.
const PIXELS: [char; 15] = [
    ' ', '.', ',', ':', ';', 'i', '1', 't', 'f', 'L', 'C', 'G', 'O', '8', '@',
];

/// Returns the ASCII character that best represents the given RGB color.
pub fn to_ascii(rgba: &image::LumaA<u8>) -> char {
    if rgba.0[1] == 0 {
        return PIXELS[0];
    }

    // Normalize the intensity value to the range [0, 1].
    let intensity = rgba.to_luma().0[0] as f32 / 255.0;

    debug_assert!((0.0..=1.0).contains(&intensity),);

    // Calculate the index of the ASCII character that best represents the intensity value.
    let index = (intensity * (PIXELS.len() - 1) as f32).round() as usize;

    PIXELS[index]
}
