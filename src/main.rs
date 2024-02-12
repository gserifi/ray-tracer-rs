use rand::prelude::*;

mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::camera::Camera;
use crate::color::ColorWriter;
use crate::hittable::{Hittable, HittableList};
use crate::sphere::Sphere;
use crate::vec3::{Point3, XYZAccessor};

fn main() {
    // Random Number Generator
    let mut rng = thread_rng();

    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let mut cam = Camera::new(&mut rng);
    cam.image_width = 1920;
    cam.samples_per_pixel = 100;

    cam.render(&world);
}
