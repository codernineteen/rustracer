use raytracer::components::camera::{self, Camera};
fn main() {
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let depth = 10;

    let camera = Camera::new(image_width, aspect_ratio, samples_per_pixel, depth);
    camera.write_color();
}
