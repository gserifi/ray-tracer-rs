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

enum RenderMode {
    QUICK,
    ULTRA,
}

fn main() {
    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let mut cam = Camera::new();
    let render_mode = RenderMode::ULTRA;

    match render_mode {
        RenderMode::QUICK => {
            cam.image_width = 1080;
            cam.samples_per_pixel = 100;
            cam.max_depth = 50;
        }
        RenderMode::ULTRA => {
            cam.image_width = 3840;
            cam.samples_per_pixel = 200;
            cam.max_depth = 100;
        }
    }

    cam.render(&world);
}
