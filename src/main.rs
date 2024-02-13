use image::RgbImage;
use std::env;
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

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
use crate::RenderMode::{Dev, Latest};

fn render(render_mode: RenderMode) -> RgbImage {
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

    match render_mode {
        Dev => {
            cam.image_width = 1080;
            cam.samples_per_pixel = 12;
            cam.max_depth = 50;
            cam.focal_length = 1.0;
        }
        Latest => {
            cam.image_width = 3840;
            cam.samples_per_pixel = 25;
            cam.max_depth = 50;
            cam.focal_length = 1.0;
        }
    }

    cam.render(&world)
}

#[derive(Clone)]
enum RenderMode {
    Dev,
    Latest,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let render_mode = if args.len() > 1 {
        match args[1].as_str() {
            "dev" => Dev,
            "latest" => Latest,
            _ => panic!("Invalid argument"),
        }
    } else {
        Dev
    };

    let output_path = match render_mode {
        Dev => Path::new("images/output.png"),
        Latest => Path::new("latest.png"),
    };

    let (tx, rx) = mpsc::channel();
    let n_threads = 10;

    for _ in 0..n_threads {
        let tx = tx.clone();
        let render_mode = render_mode.clone();
        thread::spawn(move || {
            let image = render(render_mode);
            tx.send(image).unwrap();
        });
    }

    let images: Vec<_> = rx.iter().take(n_threads).collect();

    let mut output_image: RgbImage = RgbImage::new(images[0].width(), images[0].height());

    Camera::aggregate(&mut output_image, &images);
    output_image.save(output_path).unwrap();
}
