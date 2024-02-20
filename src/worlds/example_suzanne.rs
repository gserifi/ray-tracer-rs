use std::rc::Rc;

use crate::geometry::{Sphere, TriangleMesh, World};
use crate::materials::{Dielectric, Metal};
use crate::optics::{LensConfig, ViewportConfig};
use crate::utils::{Point3, Vec3};

pub fn example_suzanne() -> (World, ViewportConfig, LensConfig) {
    // Materials
    let material_ground = Rc::new(Metal::new(Vec3::new(0.6, 0.6, 0.8) * 0.7, 0.0));
    // let material_suzanne = Rc::new(Normal::new());
    let material_suzanne = Rc::new(Dielectric::frosted(1.5, 0.1));

    // Objects
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );

    let suzanne = TriangleMesh::new(
        "assets/meshes/suzanne_smooth.obj",
        Point3::new(0.0, 0.8, 0.0),
        1.0,
        material_suzanne.clone(),
    );

    // World
    let world = World::new(vec![Rc::new(ground), Rc::new(suzanne)]);

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
