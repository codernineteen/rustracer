use crate::linalg::{point::Point3, ray::Ray, vector::Vector3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vector3,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the normal vector of the hit record based on the ray and the outward normal.
    /// Assuming that the lengths of the ray and the outward normal are 1, the dot product of two vectors is equal to the cosine of the angle between them.
    /// If the angle is less than 90 degrees, value of the cosine is larger than 0.0 and it means that the ray and the outward normal are pointing in the same direction, so the ray is inside the object.
    /// This function is required by an effect like refraction
    ///
    /// # Arguments
    ///
    /// * `r` - The ray that hit the object.
    /// * `outward_normal` - The outward normal of the object.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        if Vector3::dot(&r.direction, &outward_normal) > 0.0 {
            // back face of a certain surface -> invert direction of normal so that it can works as a front face always.
            self.front_face = false;
            self.normal = -outward_normal;
        } else {
            self.front_face = true;
            self.normal = outward_normal;
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            t: 0.0,
            normal: Vector3::new(0.0, 0.0, 1.0),
            front_face: true,
        }
    }
}
