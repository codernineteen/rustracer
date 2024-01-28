use crate::{
    geometry::{geometry_trait::Hittable, sphere::Sphere},
    linalg::{color::*, point::Point3, ray::Ray, vector::Vector3},
};
use image::{ImageBuffer, RgbImage};

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u8,
    pub viewport_width: f64,
    pub viewport_height: f64,
    camera_center: Point3,
    pixel_u: Vector3,
    pixel_v: Vector3,
    pixel_origin: Point3,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_depth: u8,
        viewport_height: f64,
        focal_length: f64,
        camera_center: Point3,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_u = viewport_u / image_width as f64;
        let pixel_v = viewport_v / image_height as f64;

        let viewport_top_left = camera_center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel_origin = viewport_top_left + 0.5 * (pixel_u + pixel_v);

        Self {
            image_width,
            image_height,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            viewport_width,
            viewport_height,
            camera_center, // default origin
            pixel_u,
            pixel_v,
            pixel_origin,
        }
    }

    pub fn render_picture(&self) {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        for i in 0..self.image_height {
            print!("\r[info] : scanlines remaining : {}", i);
            for j in 0..self.image_width {
                let pixel_loc =
                    self.pixel_origin + (self.pixel_u * j as f64) + (self.pixel_v * i as f64);
                let ray_direction = pixel_loc - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);

                let pixel_color = Self::ray_color(&ray, &sphere);
                write_color(&mut img, pixel_color, i, j);
            }
        }

        if let Err(e) = img.save("output.png") {
            eprint!("[error] : failed to save image - {}\n", e);
        } else {
            print!("\r[info] : successfully saved an output image to local directory.\n")
        }
    }

    pub fn ray_color(r: &Ray, sphere: &Sphere) -> Color3 {
        let t = Sphere::hit(&r, &sphere);

        if t > 0.0 {
            let normal = sphere.normal(r.at(t));
            0.5 * (normal + Color3::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = r.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0)
        }
    }
}
