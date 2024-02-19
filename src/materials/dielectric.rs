use rand::prelude::{Rng, ThreadRng};

use crate::geometry::HitRecord;
use crate::materials::Material;
use crate::optics::Ray;
use crate::utils::{Vec3, Vec3Ext};

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Self::new(1.5)
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = (-r_in.direction()).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            r_in.direction().reflect(&rec.normal)
        } else {
            r_in.direction().refract(&rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction, r_in.time());
        true
    }
}
