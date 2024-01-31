use crate::{
    geometry::{geometry_trait::Hittable, world::World},
    linalg::{color::*, point::Point3, ray::Ray, vector::Vector3},
    util::{hit_record::HitRecord, interval::Interval},
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

        let pixel_u = viewport_u / (image_width as f64);
        let pixel_v = viewport_v / (image_height as f64);

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

    pub fn render_picture(&self, world: World) {
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        for i in 0..self.image_height {
            print!("\r[info] : scanlines remaining : {}", self.image_height - i);
            for j in 0..self.image_width {
                let mut pixel_color = Color3::default();
                let pixel_loc =
                    self.pixel_origin + (self.pixel_u * j as f64) + (self.pixel_v * i as f64);
                for _ in 0..self.samples_per_pixel {
                    self.antialias(&mut pixel_color, &pixel_loc, &world);
                }
                write_color(&mut img, pixel_color, i, j, self.samples_per_pixel);
            }
        }

        if let Err(e) = img.save("output.png") {
            eprint!("[error] : failed to save image - {}\n", e);
        } else {
            print!("\r[info] : successfully saved an output image to local directory.\n")
        }
    }

    pub fn antialias(&self, pixel_color: &mut Color3, pixel_loc: &Vector3, world: &World) {
        let random_x = rand::random::<f64>() - 0.5; // [-0.5, 0.5)
        let random_y = rand::random::<f64>() - 0.5; // [-0.5, 0.5)
        let pixel_sample_loc =
            pixel_loc.clone() + (self.pixel_u * random_x) + (self.pixel_v * random_y);

        let sample_ray = Ray::new(self.camera_centerc, pixel_sample_loc - self.camera_center);
        *pixel_color = *pixel_color + Self::ray_color(&sample_ray, &world); // culmulate  the color
    }

    pub fn ray_color(r: &Ray, world: &World) -> Color3 {
        let mut interval = Interval::new(0.0, f64::INFINITY);
        let mut hit_record = HitRecord::default();

        if world.hit(r, &mut interval, &mut hit_record) {
            0.5 * (hit_record.normal + Color3::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = r.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0)
        }
    }
}
