use std::env;
use std::path::Path;

mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::sphere::Sphere;
use crate::vec3::{Point3, XYZAccessor};

fn main() {
    let args: Vec<String> = env::args().collect();
    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let mut cam = Camera::new();
    let output_path: &Path;

    if &args[1] == "dev" {
        cam.image_width = 1080;
        cam.samples_per_pixel = 100;
        cam.max_depth = 50;
        output_path = Path::new("images/output.png");
    } else if &args[1] == "latest" {
        cam.image_width = 3840;
        cam.samples_per_pixel = 200;
        cam.max_depth = 100;
        output_path = Path::new("latest.png");
    } else {
        panic!("Invalid argument");
    }

    cam.render(&world, output_path);
}
