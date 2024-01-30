use raytracer::check_func_perf;
use raytracer::components::camera::Camera;
use raytracer::geometry::sphere::Sphere;
use raytracer::geometry::world::World;
use raytracer::linalg::point::Point3;
use std::sync::Arc;

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
    let mut world = World::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    //world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 95.0)));

    check_func_perf!(camera.render_picture(world)); // move ownership of world to render_picture
}
