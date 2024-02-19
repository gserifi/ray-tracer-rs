use std::rc::Rc;

use crate::geometry::{Sphere, World};
use crate::materials::{Lambertian, Normal};
use crate::optics::{LensConfig, ViewportConfig};
use crate::utils::{Point3, Vec3};

pub fn example_normal() -> (World, ViewportConfig, LensConfig) {
    // Materials
    let material_ground = Rc::new(Lambertian::from_albedo(Vec3::new(0.03, 0.03, 0.03)));
    let material_center = Rc::new(Normal::new());

    // Objects
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );

    let center_sphere = Sphere::new(Point3::new(0.0, 1.0, 0.0), 0.5, material_center.clone());

    // World
    let world = World::new(vec![Rc::new(ground), Rc::new(center_sphere)]);

    let alpha: f64 = 0.0;
    let viewport_config = ViewportConfig {
        vertical_fov: 20.0,
        look_from: Point3::new(
            4.0 * alpha.to_radians().cos(),
            1.0,
            4.0 * alpha.to_radians().sin(),
        ),
        look_at: Point3::new(0.0, 1.0, 0.0),
        view_up: Vec3::new(0.0, 1.0, 0.0),
    };

    let lens_config = LensConfig {
        depth_of_field_angle: 0.0,
        focus_dist: 1.0,
    };

    (world, viewport_config, lens_config)
}
