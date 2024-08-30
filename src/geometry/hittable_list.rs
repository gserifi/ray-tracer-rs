use std::fmt::Debug;

use crate::geometry::{accel::AABB, HitRecord, Hittable};
use crate::optics::Ray;
use crate::utils::Interval;

pub type World = HittableList;

#[derive(Debug, Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    pub bbox: AABB,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        let bbox = AABB::wrap_objects(&objects);

        Self { objects, bbox }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = if self.objects.is_empty() {
            object.bounding_box().clone()
        } else {
            AABB::wrap_boxes(&self.bbox, object.bounding_box())
        };
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t.max;

        if !self.bbox.hit(r, t, rec) {
            return false;
        }

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
