use raytracer::check_func_perf;
use raytracer::components::camera::Camera;
use raytracer::linalg::point::Point3;

fn main() {
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let depth = 10;
    let viewport_height = 2.0;
    let camera_center = Point3::default();
    let focal_length = 1.0;

    let camera = Camera::new(
        image_width,
        aspect_ratio,
        samples_per_pixel,
        depth,
        viewport_height,
        focal_length,
        camera_center,
    );
    check_func_perf!(camera.render_picture());
}
