use super::geometry_trait::Hittable;
use crate::{
    linalg::ray::Ray,
    util::{hit_record::HitRecord, interval::Interval},
};
use std::sync::Arc;

pub struct World {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for World {
    fn hit(&self, r: &Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_t = interval.max;

        // find the closest intersection with current ray.
        for object in self.objects.iter() {
            interval.update(interval.min, closest_t);
            if object.hit(r, interval, &mut temp_record) {
                hit_anything = true;
                closest_t = temp_record.t;
                hit_record.clone_from(&temp_record);
            }
        }

        hit_anything
    }
}
