use rand::prelude::ThreadRng;
use std::rc::Rc;

use crate::geometry::HitRecord;
use crate::materials::Material;
use crate::optics::Ray;
use crate::textures::{Solid, Texture};
use crate::utils::{Vec3, Vec3Ext};

#[derive(Debug)]
pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn from_albedo(albedo: Vec3) -> Self {
        Self::from_texture(Rc::new(Solid::new(albedo)))
    }
    pub fn from_texture(texture: Rc<dyn Texture>) -> Self {
        Self { albedo: texture }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self::from_albedo(Vec3::new(0.5, 0.5, 0.5))
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
        *attenuation = self.albedo.sample(rec.u, rec.v, &rec.p);
        true
    }
}
