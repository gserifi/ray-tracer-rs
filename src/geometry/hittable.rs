use crate::geometry::{accel::AABB, HitRecord};
use crate::optics::Ray;
use crate::utils::Interval;

pub trait Hittable {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AABB;

    fn depth(&self) -> usize {
        1
    }
}
