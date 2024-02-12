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
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

fn main() {
    let args: Vec<String> = env::args().collect();
    // World

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.4, 0.4, 0.5))) as Rc<dyn Material>;
    let material_center = Rc::new(Dielectric::new(1.5)) as Rc<dyn Material>;
    let material_left = Rc::new(Metal::new(Vec3::new(0.8, 0.3, 0.3), 0.02)) as Rc<dyn Material>;
    let material_right = Rc::new(Metal::new(Vec3::new(0.3, 0.3, 0.8), 0.3)) as Rc<dyn Material>;

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.05, -0.9),
        0.5,
        Rc::clone(&material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.05, -0.9),
        -0.4,
        Rc::clone(&material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-0.75, 0.3, -1.6),
        0.5,
        Rc::clone(&material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.75, 0.0, -1.6),
        0.5,
        Rc::clone(&material_right),
    )));

    // Camera

    let mut cam = Camera::new();
    let output_path: &Path;

    if &args[1] == "dev" {
        cam.image_width = 1080;
        cam.samples_per_pixel = 100;
        cam.max_depth = 50;
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
