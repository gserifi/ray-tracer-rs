use crate::vec3::Vec3;
use rand::prelude::*;
use rand_distr::StandardNormal;

pub fn random_uniform_vector(rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(rng.gen(), rng.gen(), rng.gen())
}

pub fn random_gaussian_vector(rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(
        rng.sample(StandardNormal),
        rng.sample(StandardNormal),
        rng.sample(StandardNormal),
    )
}

pub fn random_unit_sphere_vector(rng: &mut ThreadRng) -> Vec3 {
    let v = random_gaussian_vector(rng);
    v.normalize()
}
