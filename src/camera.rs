use rand::prelude::*;

use crate::color::{Color, ColorWriter};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, XYZAccessor};

pub struct Camera<'a> {
    rng: &'a mut ThreadRng,

    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,

    // Derived
    image_height: i32,
    origin: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl<'a> Camera<'a> {
    pub fn new(rng: &'a mut ThreadRng) -> Self {
        Self {
            rng,
            aspect_ratio: 16.0 / 9.0,
            image_width: 100,
            samples_per_pixel: 10,

            // Derived
            image_height: 0,
            origin: Point3::zeros(),
            pixel00_loc: Point3::zeros(),
            pixel_delta_u: Vec3::zeros(),
            pixel_delta_v: Vec3::zeros(),
        }
    }
}

impl Camera<'_> {
    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        println!("P3\n{} {}\n{}", self.image_width, self.image_height, 255);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::zeros();

                // Super Sampling
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world);
                }
                println!("{}", pixel_color.write_color(self.samples_per_pixel));
            }

            eprint!(
                "\rRendering Output: {:.1}% completed",
                (j as f64 / (self.image_height - 1) as f64) * 100.0
            );
        }
        eprintln!("\rDone.                                  \n");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let focal_length = 1.0;
        let camera_origin = Vec3::zeros();

        // Viewport Vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            camera_origin - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.0;
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::empty();
        if world.hit(r, Interval::right_open(0.0), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direction = pixel_sample - self.origin;

        Ray::new(self.origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();

        px * self.pixel_delta_u + py * self.pixel_delta_v
    }
}