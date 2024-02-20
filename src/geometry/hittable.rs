use std::fmt::Debug;

use crate::geometry::{accel::AABB, HitRecord};
use crate::optics::Ray;
use crate::utils::Interval;

pub trait Hittable: Debug {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AABB;
    fn bounding_box_owned(&self) -> AABB {
        self.bounding_box().clone()
    }

    fn height(&self) -> usize {
        1
    }
}
