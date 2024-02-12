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
    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let mut cam = Camera::new();
    // 8K: 7680 x 4320, 4K: 3840 x 2160, FullHD: 1920 x 1080
    cam.image_width = 1080;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
