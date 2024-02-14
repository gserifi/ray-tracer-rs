use rand::prelude::ThreadRng;

use crate::geometry::HitRecord;
use crate::optics::Ray;
use crate::utils::Vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
