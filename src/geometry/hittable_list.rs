use std::fmt::Debug;
use std::rc::Rc;

use crate::geometry::{accel::AABB, HitRecord, Hittable};
use crate::optics::Ray;
use crate::utils::Interval;

pub type World = HittableList;

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new(objects: Vec<Rc<dyn Hittable>>) -> Self {
        let bbox = AABB::wrap_objects(&objects);

        Self { objects, bbox }
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::wrap_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t.max;

        if !self.bbox.hit(r, t) {
            return false;
        }

        for object in &self.objects {
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
}
