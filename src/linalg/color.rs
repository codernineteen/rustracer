use crate::linalg::vector::Vector3;
use image::{Rgb, RgbImage};

/*
    logical alias of Vector3
    Fields follow this mapping :
    x == r
    y == g
    z == b
*/
pub type Color3 = Vector3;

pub fn write_color(img: &mut RgbImage, pixel: Color3, row: u32, col: u32) {
    let pixel = Rgb([
        (pixel.x * 255.999) as u8,
        (pixel.y * 255.999) as u8,
        (pixel.z * 255.999) as u8,
    ]);

    img.put_pixel(col, row, pixel); // width | height | pixel value
}
