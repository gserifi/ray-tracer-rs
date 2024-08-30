use std::fmt::Debug;

use crate::geometry::{accel::AABB, HitRecord};
use crate::optics::Ray;
use crate::utils::Interval;

pub trait Hittable: Debug {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AABB;

    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
