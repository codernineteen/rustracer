use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u8,
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64, samples_per_pixel: u32, max_depth: u8) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }

        Self {
            image_width,
            image_height,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render_picture(&self) {
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        for i in 0..self.image_height - 1 {
            print!("\r[info] : scanlines remaining : {}", i);
            for j in 0..self.image_width - 1 {
                let r = (j as f32 / (self.image_width - 1) as f32) * 255.999;
                let g = (i as f32 / (self.image_height - 1) as f32) * 255.999;
                let b = 0;

                let pixel = Rgb([r as u8, g as u8, b]);

                img.put_pixel(j, i, pixel); // width | height | pixel value
            }
        }

        if let Err(e) = img.save("output.png") {
            eprint!("[error] : failed to save image - {}\n", e);
        } else {
            print!("\r[info] : successfully saved an output image to local directory.\n")
        }
    }
}
