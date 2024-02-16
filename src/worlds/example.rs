use std::rc::Rc;

use crate::geometry::{Sphere, World};
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::optics::{LensConfig, ViewportConfig};
use crate::utils::{Point3, Vec3};

pub fn example_world() -> (World, ViewportConfig, LensConfig) {
    // Materials
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.03, 0.03, 0.03))) as Rc<dyn Material>;
    let material_center = Rc::new(Dielectric::new(1.5)) as Rc<dyn Material>;
    let material_left = Rc::new(Lambertian::new(Vec3::new(0.8, 0.9, 1.0))) as Rc<dyn Material>;
    let material_right = Rc::new(Metal::new(Vec3::new(0.95, 0.95, 0.95), 0.0)) as Rc<dyn Material>;

    // Objects
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    );

    let center_sphere = Sphere::new(
        Point3::new(0.0, 0.01, -1.0),
        0.5,
        Rc::clone(&material_center),
    );

    let center_inner_sphere = Sphere::new(
        Point3::new(0.0, 0.01, -1.0),
        -0.4,
        Rc::clone(&material_center),
    );

    let left_sphere = Sphere::new(
        Point3::new(-1.01, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    );

    let right_sphere = Sphere::new(
        Point3::new(1.01, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right),
    );

    // World
    let world = World::new(vec![
        Box::new(ground),
        Box::new(center_sphere),
        Box::new(center_inner_sphere),
        Box::new(left_sphere),
        Box::new(right_sphere),
    ]);

    let viewport_config = ViewportConfig {
        vertical_fov: 20.0,
        look_from: Point3::new(-2.0, 2.0, 1.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
        view_up: Vec3::new(0.0, 1.0, 0.0),
    };

    let lens_config = LensConfig {
        depth_of_field_angle: 3.0,
        focus_dist: 3.4,
    };

    (world, viewport_config, lens_config)
}
