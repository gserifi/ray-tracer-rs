use rand::prelude::ThreadRng;

use crate::geometry::HitRecord;
use crate::materials::Material;
use crate::optics::Ray;
use crate::utils::{Vec3, Vec3Ext};

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self::new(Vec3::new(0.8, 0.8, 0.8), 0.0)
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction().reflect(&rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_unit_sphere_vector(rng),
            r_in.time(),
        );
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0
    }
}
