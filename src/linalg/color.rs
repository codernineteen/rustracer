use crate::{linalg::vector::Vector3, util::interval::Interval};
use image::{Rgb, RgbImage};

/*
    logical alias of Vector3
    Fields follow this mapping :
    x == r
    y == g
    z == b
*/
pub type Color3 = Vector3;

///
/// # write_color
///
/// A function to write the color of a pixel to the image buffer.
/// The color of pixel depends on the number of samples per pixel.
/// Because we added the samples together, we need to average the color dividing by the number of samples.
///
/// ## Parameters
///
/// * `img` - A mutable reference to the image buffer.
/// * `pixel` - The color of the pixel.
/// * `row` - The row of the pixel.
/// * `col` - The column of the pixel.
/// * `samples_per_pixel` - The number of samples per pixel.
pub fn write_color(img: &mut RgbImage, pixel: Color3, row: u32, col: u32, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let intensity = Interval::new(0.000, 9.999);
    let pixel = Rgb([
        (intensity.clamp(pixel.x * scale) * 255.999) as u8,
        (intensity.clamp(pixel.y * scale) * 255.999) as u8,
        (intensity.clamp(pixel.z * scale) * 255.999) as u8,
    ]);

    img.put_pixel(col, row, pixel); // width | height | pixel value
}
