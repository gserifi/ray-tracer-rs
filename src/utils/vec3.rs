extern crate nalgebra as na;

use na::Vector3 as _Vector3;
use rand::prelude::{Rng, ThreadRng};
use rand_distr::StandardNormal;
use std::ops::Index;

pub type Vec3 = _Vector3<f64>;
pub type Point3 = Vec3;

pub trait RGBAccessor {}

pub trait Vec3Ext
where
    Self: Index<usize, Output = f64>,
{
    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn z(&self) -> f64 {
        self[2]
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    fn reflect(&self, n: &Vec3) -> Vec3;

    fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3;

    fn random_uniform_vector(rng: &mut ThreadRng) -> Vec3 {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    fn random_gaussian_vector(rng: &mut ThreadRng) -> Vec3 {
        Vec3::new(
            rng.sample(StandardNormal),
            rng.sample(StandardNormal),
            rng.sample(StandardNormal),
        )
    }
    fn random_unit_sphere_vector(rng: &mut ThreadRng) -> Vec3 {
        Self::random_gaussian_vector(rng).normalize()
    }

    fn random_hemisphere_vector(rng: &mut ThreadRng, normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_sphere_vector(rng);
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    fn random_unit_disk_vector(rng: &mut ThreadRng) -> Vec3 {
        let r = rng.gen::<f64>().sqrt();
        let theta = 2.0 * std::f64::consts::PI * rng.gen::<f64>();

        Vec3::new(r * theta.cos(), r * theta.sin(), 0.0)
    }
}

impl Vec3Ext for Vec3 {
    fn reflect(&self, n: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}
