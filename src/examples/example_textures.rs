// use std::rc::Rc;
//
// use crate::geometry::{Sphere, World};
// use crate::materials::{Dielectric, Lambertian};
// use crate::optics::{LensConfig, ViewportConfig};
// use crate::textures::{UVImage, XYZChecker};
// use crate::utils::{Color, Point3, Vec3};
//
// pub fn example_textures() -> (World, ViewportConfig, LensConfig) {
//     // Textures
//     let checker1 = Rc::new(XYZChecker::from_colors(
//         0.3,
//         Color::new(0.2, 0.3, 0.1),
//         Color::new(0.9, 0.9, 0.9),
//     ));
//
//     // let checker2 = Rc::new(UVChecker::from_colors(
//     //     0.1,
//     //     Color::new(0.3, 0.1, 0.1),
//     //     Color::new(0.7, 0.7, 0.7),
//     // ));
//
//     let image_tex = Rc::new(UVImage::new("assets/textures/earthmap.jpg"));
//
//     // let checker3 = Rc::new(XYZChecker::from_colors(
//     //     0.1,
//     //     Color::new(0.3, 0.3, 0.3),
//     //     Color::new(0.7, 0.7, 0.7),
//     // ));
//
//     // Materials
//     let material_ground = Rc::new(Lambertian::from_texture(checker1.clone()));
//     let material_center = Rc::new(Lambertian::from_texture(image_tex.clone()));
//     let material_center2 = Rc::new(Dielectric::new(1.5));
//     // let material_suzanne = Rc::new(Lambertian::from_texture(checker3.clone()));
//
//     // Objects
//     let ground = Sphere::new(
//         Point3::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground.clone(),
//     );
//
//     let center_sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.5, material_center.clone());
//     let center_sphere_outer =
//         Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.503, material_center2.clone());
//     let center_sphere_inner =
//         Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.502, material_center2.clone());
//
//     // let suzanne = TriangleMesh::new(
//     //     "assets/meshes/suzanne.obj",
//     //     Point3::new(-0.3, 0.0, -1.2),
//     //     0.5,
//     //     material_suzanne.clone(),
//     // );
//
//     // World
//     let world = World::new(vec![
//         Rc::new(ground),
//         Rc::new(center_sphere),
//         Rc::new(center_sphere_outer),
//         Rc::new(center_sphere_inner),
//         // Rc::new(suzanne),
//     ]);
//
//     let angle: f64 = 0.0;
//     let viewport_config = ViewportConfig {
//         vertical_fov: 25.0,
//         look_from: Point3::new(
//             2.0 * angle.to_radians().cos(),
//             1.5,
//             2.0 * angle.to_radians().sin(),
//         ),
//         look_at: Point3::new(0.0, 0.0, 0.0),
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
