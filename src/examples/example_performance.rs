// use std::rc::Rc;
//
// use crate::geometry::{accel::BvhNode, Hittable, Sphere, World};
// use crate::materials::Lambertian;
// use crate::optics::{LensConfig, ViewportConfig};
// use crate::utils::{Point3, Vec3};
//
// pub fn example_performance() -> (World, ViewportConfig, LensConfig) {
//     let material_ground = Rc::new(Lambertian::from_albedo(Vec3::new(0.03, 0.03, 0.03)));
//     let material_sphere = Rc::new(Lambertian::from_albedo(Vec3::new(0.2, 0.2, 0.7)));
//
//     // Objects
//     let ground = Sphere::new(
//         Point3::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground.clone(),
//     );
//
//     let mut objects: Vec<Rc<dyn Hittable>> = vec![];
//
//     for i in -15..=15 {
//         for j in -15..=3 {
//             let (x, z) = (i as f64 / 2.25, j as f64 / 2.25);
//             let center = Point3::new(x, x.sin() * z.sin() + 1.0, z);
//             let sphere = Sphere::new(center, 0.2, material_sphere.clone());
//             objects.push(Rc::new(sphere));
//         }
//     }
//
//     // World
//     let world = BvhNode::new(&mut objects);
//     let world = World::new(vec![Rc::new(ground), Rc::new(world)]);
//
//     let viewport_config = ViewportConfig {
//         vertical_fov: 70.0,
//         look_from: Point3::new(0.0, 6.0, 3.5),
//         look_at: Point3::new(0.0, 1.0, -1.0),
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
