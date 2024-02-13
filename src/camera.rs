use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use rand::prelude::*;
use std::path::Path;
use tqdm::tqdm;

use crate::color::{color_to_rgb8, linear_to_gamma, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_unit_sphere_vector;
use crate::vec3::{Point3, Vec3, XYZAccessor};

pub struct Camera {
    rng: ThreadRng,

    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub focal_length: f64,

    // Derived
    image_height: u32,
    origin: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
            aspect_ratio: 16.0 / 9.0,
            image_width: 100,
            samples_per_pixel: 100,
            max_depth: 50,
            focal_length: 1.0,

            // Derived
            image_height: 0,
            origin: Point3::zeros(),
            pixel00_loc: Point3::zeros(),
            pixel_delta_u: Vec3::zeros(),
            pixel_delta_v: Vec3::zeros(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) -> RgbImage {
        self.initialize();
        let mut output_image: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        for j in tqdm(0..self.image_height) {
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::zeros();

                // Super Sampling
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                pixel_color /= self.samples_per_pixel as f64;
                // pixel_color = linear_to_gamma(&pixel_color);

                output_image.put_pixel(i, j, color_to_rgb8(&pixel_color));
            }
        }

        output_image
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let camera_origin = Vec3::zeros();

        // Viewport Vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = camera_origin
            - Vec3::new(0.0, 0.0, self.focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.0;
    }

    fn ray_color(&mut self, r: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::zeros();
        }

        let mut rec = HitRecord::empty();
        if world.hit(r, Interval::right_open(0.001), &mut rec) {
            let mut scattered = Ray::new(Point3::zeros(), Vec3::new(1.0, 0.0, 0.0));
            let mut attenuation = Color::zeros();

            if rec
                .mat
                .scatter(r, &rec, &mut self.rng, &mut attenuation, &mut scattered)
            {
                return attenuation.component_mul(&self.ray_color(&scattered, depth - 1, world));
            }

            return Color::zeros();
        }

        let unit_direction = r.direction();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
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

impl Camera {
    pub fn aggregate(base_image: &mut RgbImage, images: &Vec<RgbImage>) {
        for (x, y, pixel) in tqdm(base_image.enumerate_pixels_mut()) {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for image in images {
                let pixel_color = image.get_pixel(x, y);
                color += Color::new(
                    pixel_color[0] as f64 / 255.0,
                    pixel_color[1] as f64 / 255.0,
                    pixel_color[2] as f64 / 255.0,
                ) / images.len() as f64;
            }

            color = linear_to_gamma(&color);
            *pixel = color_to_rgb8(&color);
        }
    }
}
