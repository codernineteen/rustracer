use crate::{
    linalg::ray::Ray,
    util::{hit_record::HitRecord, interval::Interval},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool;
}
