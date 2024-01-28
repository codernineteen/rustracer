use crate::linalg::ray::Ray;

pub trait Hittable<T> {
    fn hit(r: &Ray, object: &T) -> f64;
}
