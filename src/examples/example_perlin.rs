use crate::geometry::{Sphere, World};
use crate::materials::Lambertian;
use crate::optics::{LensConfig, ViewportConfig};
use crate::textures::PerlinNoise;
use crate::utils::{Point3, Vec3};
use std::rc::Rc;

pub fn example_perlin() -> (World, ViewportConfig, LensConfig) {
    // Materials
    let perlin = Rc::new(PerlinNoise::new_bw(4.0, 5.0));
    // let perlin = Rc::new(PerlinNoise::new(
    //     4.0,
    //     Color::new(0.8, 0.8, 0.3),
    //     Color::new(0.8, 0.3, 0.8),
    // ));
    let material_ground = Rc::new(Lambertian::from_texture(perlin.clone()));
    let material_sphere = Rc::new(Lambertian::from_texture(perlin.clone()));

    // Objects
    let ground = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    );

    let sphere = Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, material_sphere.clone());

    // World
    let world = World::new(vec![Box::new(ground), Box::new(sphere)]);

    let viewport_config = ViewportConfig {
        vertical_fov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        view_up: Vec3::new(0.0, 1.0, 0.0),
    };

    let lens_config = LensConfig {
        depth_of_field_angle: 0.0,
        focus_dist: 1.0,
    };

    (world, viewport_config, lens_config)
}
