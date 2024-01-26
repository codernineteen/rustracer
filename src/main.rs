use raytracer::check_func_perf;
use raytracer::components::camera::Camera;

fn main() {
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let depth = 10;

    let camera = Camera::new(image_width, aspect_ratio, samples_per_pixel, depth);
    check_func_perf!(camera.render_picture());
}
