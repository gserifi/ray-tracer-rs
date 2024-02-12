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
    random_gaussian_vector(rng).normalize()
}

pub fn random_hemisphere_vector(rng: &mut ThreadRng, normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_sphere_vector(rng);
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
