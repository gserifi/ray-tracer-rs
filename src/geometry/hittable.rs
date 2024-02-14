use crate::geometry::HitRecord;
use crate::optics::Ray;
use crate::utils::Interval;

pub trait Hittable {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool;
}
