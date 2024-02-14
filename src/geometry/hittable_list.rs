use crate::geometry::{HitRecord, Hittable};
use crate::optics::Ray;
use crate::utils::Interval;

pub type World = HittableList;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
