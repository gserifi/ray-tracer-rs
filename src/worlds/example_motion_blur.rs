use std::rc::Rc;

use crate::geometry::{Sphere, World};
use crate::materials::{Dielectric, Lambertian, Material};
use crate::optics::{LensConfig, ViewportConfig};
use crate::utils::{Point3, Vec3};

pub fn example_motion_blur() -> (World, ViewportConfig, LensConfig) {
    // Materials
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.1, 0.1, 0.2))) as Rc<dyn Material>;
    let material_sphere = Rc::new(Lambertian::new(Vec3::new(0.7, 0.1, 0.1))) as Rc<dyn Material>;

    // Objects
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    );

    let sphere = Sphere::new_moving(
        Point3::new(-0.1, 0.0, -1.0),
        Point3::new(0.1, 0.1, -1.0),
        0.5,
        Rc::clone(&material_sphere),
    );

    // World
    let world = World::new(vec![Box::new(ground), Box::new(sphere)]);

    let viewport_config = ViewportConfig {
        vertical_fov: 90.0,
        look_from: Point3::new(0.0, 0.0, 0.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
        view_up: Vec3::new(0.0, 1.0, 0.0),
    };

    let lens_config = LensConfig {
        depth_of_field_angle: 0.0,
        focus_dist: 3.4,
    };

    (world, viewport_config, lens_config)
}
