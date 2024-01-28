use crate::linalg::{point::Point3, ray::Ray, vector::Vector3};

use super::geometry_trait::Hittable;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn normal(&self, p: Point3) -> Vector3 {
        (p - self.center).normalize()
    }
}

impl Hittable<Sphere> for Sphere {
    fn hit(r: &Ray, object: &Sphere) -> f64 {
        let d = r.direction; //
        let oc = r.origin - object.center;
        // discriminant = (b/2 - sqrt(b^2-ac)) / a
        let b = 1.0 * Vector3::dot(&d, &oc);
        let a = Vector3::dot(&d, &d);
        let c = Vector3::dot(&oc, &oc) - object.radius * object.radius;
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / a
        }
    }
}
