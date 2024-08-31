use std::rc::Rc;

use crate::geometry::{Quad, World};
use crate::materials::Lambertian;
use crate::optics::{LensConfig, ViewportConfig};
use crate::utils::{Color, Point3, Vec3};

pub fn example_quads() -> (World, ViewportConfig, LensConfig) {
    // Materials
    let left_red = Rc::new(Lambertian::from_albedo(Color::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::from_albedo(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::from_albedo(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::from_albedo(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::from_albedo(Color::new(0.2, 0.8, 0.8)));

    // Objects
    let left_quad = Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red.clone(),
    );

    let back_quad = Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green.clone(),
    );

    let right_quad = Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue.clone(),
    );

    let upper_quad = Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange.clone(),
    );

    let lower_quad = Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal.clone(),
    );

    // World
    let world = World::new(vec![
        Box::new(left_quad),
        Box::new(back_quad),
        Box::new(right_quad),
        Box::new(upper_quad),
        Box::new(lower_quad),
    ]);

    let viewport_config = ViewportConfig {
        vertical_fov: 80.0,
        look_from: Point3::new(0.0, 0.0, 9.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        view_up: Vec3::new(0.0, 1.0, 0.0),
    };

    let lens_config = LensConfig {
        depth_of_field_angle: 0.0,
        focus_dist: 1.0,
    };

    (world, viewport_config, lens_config)
}
