use std::rc::Rc;

use crate::geometry::{Sphere, TriangleMesh, World};
use crate::materials::{Dielectric, Lambertian, Metal};
use crate::optics::{LensConfig, ViewportConfig};
use crate::textures::PerlinNoise;
// use crate::textures::UVImage;
use crate::utils::{Color, Point3, Vec3};

pub fn example_suzanne() -> (World, ViewportConfig, LensConfig) {
    // Materials
    let perlin = Rc::new(PerlinNoise::new_bw(4.0, 20.0));
    let material_ground = Rc::new(Lambertian::from_texture(perlin.clone()));
    // let material_ground = Rc::new(Metal::new(Color::new(0.6, 0.6, 0.8) * 0.7, 0.0));
    // let material_ground = Rc::new(Lambertian::from_albedo(Color::new(0.6, 0.6, 0.8) * 0.7));
    // let material_suzanne = Rc::new(Normal::new());

    let material_suzanne = Rc::new(Dielectric::frosted(1.5, 0.01));
    // let material_suzanne = Rc::new(Dielectric::new(1.5));
    // let material_suzanne = Rc::new(Lambertian::from_texture(Rc::new(UVImage::new(
    //     "assets/textures/monkey.png",
    // ))));

    let material_glass_sphere = Rc::new(Dielectric::new(1.5));
    let material_metal_sphere = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.2), 0.0));
    let material_lambertian_sphere = Rc::new(Lambertian::from_albedo(Color::new(0.8, 0.2, 0.8)));

    // Objects
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );

    let suzanne = TriangleMesh::new(
        "assets/meshes/dragon_full.obj",
        Point3::new(1.0, 0.6, 1.0),
        4.0,
        material_suzanne.clone(),
    );

    let center_sphere = Sphere::new(
        Point3::new(3.5, 0.75, 1.75),
        0.5,
        material_glass_sphere.clone(),
    );

    let center_inner_sphere = Sphere::new(
        Point3::new(3.5, 0.75, 1.75),
        -0.4,
        material_glass_sphere.clone(),
    );

    let left_sphere = Sphere::new(
        Point3::new(-0.1, 0.35, 2.0),
        0.65,
        material_metal_sphere.clone(),
    );

    let right_sphere = Sphere::new(
        Point3::new(-0.3, 0.75, -0.5),
        0.75,
        material_lambertian_sphere.clone(),
    );

    // World
    let world = World::new(vec![
        Box::new(ground),
        Box::new(center_sphere),
        Box::new(center_inner_sphere),
        Box::new(left_sphere),
        Box::new(right_sphere),
        Box::new(suzanne),
    ]);

    let angle: f64 = 75.0;
    let viewport_config = ViewportConfig {
        vertical_fov: 40.0,
        look_from: Point3::new(
            4.0 * angle.to_radians().cos(),
            2.5,
            4.0 * angle.to_radians().sin(),
        ),
        look_at: Point3::new(0.0, 0.4, 0.0),
        view_up: Vec3::new(0.0, 1.0, 0.0),
    };

    let lens_config = LensConfig {
        depth_of_field_angle: 0.0,
        focus_dist: 1.0,
    };

    (world, viewport_config, lens_config)
}
