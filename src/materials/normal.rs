use rand::prelude::ThreadRng;

use crate::geometry::HitRecord;
use crate::materials::Material;
use crate::optics::Ray;
use crate::utils::{Vec3, Vec3Ext};

pub struct Normal {}

impl Normal {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Normal {
    fn default() -> Self {
        Self::new()
    }
}

impl Material for Normal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        _: &mut ThreadRng,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, -r_in.direction(), r_in.time());
        *attenuation = (0.5 * (rec.normal + Vec3::ones())).squared();
        true
    }
}
