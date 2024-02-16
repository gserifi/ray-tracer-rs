use rand::prelude::ThreadRng;

use crate::geometry::HitRecord;
use crate::materials::Material;
use crate::optics::Ray;
use crate::utils::{Vec3, Vec3Ext};

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self::new(Vec3::new(0.5, 0.5, 0.5))
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_sphere_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true
    }
}
