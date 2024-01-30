use crate::{
    linalg::{point::Point3, ray::Ray, vector::Vector3},
    util::{hit_record::HitRecord, interval::Interval},
};

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

    pub fn discriminate(b_half: f64, a: f64, c: f64) -> f64 {
        b_half * b_half - a * c
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let d = r.direction; //
        let oc = r.origin - self.center;
        // discriminant = (b/2 - sqrt(b^2-ac)) / a
        let b_half = Vector3::dot(&d, &oc);
        let a = r.direction.length_squared();
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = Self::discriminate(b_half, a, c);
        if discriminant < 0.0 {
            // no real roots
            return false;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-b_half - discriminant_sqrt) / a; // smaller root
        if !interval.surround(root) {
            root = (-b_half + discriminant_sqrt) / a; // bigger root
            if !interval.surround(root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        hit_record.set_face_normal(r, self.normal(hit_record.p));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discriminate() {
        let b_half = 1.0;
        let a = 1.0;
        let c = -1.0;
        let discriminant = Sphere::discriminate(b_half, a, c);
        assert_eq!(discriminant, 2.0);
    }
}
