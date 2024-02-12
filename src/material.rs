use rand::prelude::*;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_sphere_vector;
use crate::vec3::{near_zero, reflect, Vec3};

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

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + crate::utils::random_unit_sphere_vector(rng);

        if near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

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

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        _: &mut ThreadRng,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction(), &rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_unit_sphere_vector(&mut thread_rng()),
        );
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0
    }
}
