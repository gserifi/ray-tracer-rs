// use std::rc::Rc;
//
// use crate::geometry::{Sphere, World};
// use crate::materials::Lambertian;
// use crate::optics::{LensConfig, ViewportConfig};
// use crate::utils::{Point3, Vec3};
//
// pub fn example_motion_blur() -> (World, ViewportConfig, LensConfig) {
//     // Materials
//     let material_ground = Rc::new(Lambertian::from_albedo(Vec3::new(0.1, 0.1, 0.2)));
//     let material_sphere = Rc::new(Lambertian::from_albedo(Vec3::new(0.7, 0.1, 0.1)));
//
//     // Objects
//     let ground = Sphere::new(
//         Point3::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground.clone(),
//     );
//
//     let sphere = Sphere::new_moving(
//         Point3::new(-0.1, 0.0, -1.0),
//         Point3::new(0.1, 0.1, -1.0),
//         0.5,
//         material_sphere.clone(),
//     );
//
//     // World
//     let world = World::new(vec![Rc::new(ground), Rc::new(sphere)]);
//
//     let viewport_config = ViewportConfig {
//         vertical_fov: 90.0,
//         look_from: Point3::new(0.0, 0.0, 0.0),
//         look_at: Point3::new(0.0, 0.0, -1.0),
//         view_up: Vec3::new(0.0, 1.0, 0.0),
//     };
//
//     let lens_config = LensConfig {
//         depth_of_field_angle: 0.0,
//         focus_dist: 1.0,
//     };
//
//     (world, viewport_config, lens_config)
// }
