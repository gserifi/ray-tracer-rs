use std::env;
use std::path::Path;
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

fn main() {
    let args: Vec<String> = env::args().collect();
    // World

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.6)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.3, 0.8, 0.3)));
    let material_left = Rc::new(Metal::new(Vec3::new(0.8, 0.3, 0.3), 0.02));
    let material_right = Rc::new(Metal::new(Vec3::new(0.3, 0.3, 0.8), 0.3));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.1),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.1),
        0.5,
        material_right,
    )));

    // Camera

    let mut cam = Camera::new();
    let output_path: &Path;

    if &args[1] == "dev" {
        cam.image_width = 1080;
        cam.samples_per_pixel = 20;
        cam.max_depth = 5;
        cam.focal_length = 1.0;
        output_path = Path::new("images/output.png");
    } else if &args[1] == "latest" {
        cam.image_width = 3840;
        cam.samples_per_pixel = 200;
        cam.max_depth = 50;
        cam.focal_length = 1.0;
        output_path = Path::new("latest.png");
    } else {
        panic!("Invalid argument");
    }

    cam.render(&world, output_path);
}
