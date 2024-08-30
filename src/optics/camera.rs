use image::{ImageBuffer, RgbImage};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use tqdm::tqdm;

use crate::geometry::{HitRecord, Hittable};
use crate::optics::{LensConfig, Ray, RenderOutputConfig, ViewportConfig};
use crate::utils::{Color, ColorExt, Interval, Point3, Vec3, Vec3Ext};

pub struct Camera {
    rng: ThreadRng,

    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    pub vertical_fov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub view_up: Vec3,

    pub depth_of_field_angle: f64,
    pub focus_dist: f64,

    // Derived
    image_height: u32,
    origin: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,

    depth_of_field_disk_u: Vec3,
    depth_of_field_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        render_output_config: RenderOutputConfig,
        viewport_config: ViewportConfig,
        lens_config: LensConfig,
    ) -> Self {
        Self {
            rng: thread_rng(),
            aspect_ratio: render_output_config.aspect_ratio,
            image_width: render_output_config.image_width,
            samples_per_pixel: render_output_config.samples_per_pixel,
            max_depth: render_output_config.max_depth,

            vertical_fov: viewport_config.vertical_fov,
            look_from: viewport_config.look_from,
            look_at: viewport_config.look_at,
            view_up: viewport_config.view_up,

            depth_of_field_angle: lens_config.depth_of_field_angle,
            focus_dist: lens_config.focus_dist,

            // Derived
            image_height: 0,
            origin: Point3::zeros(),
            pixel00_loc: Point3::zeros(),
            pixel_delta_u: Vec3::zeros(),
            pixel_delta_v: Vec3::zeros(),

            u: Vec3::zeros(),
            v: Vec3::zeros(),
            w: Vec3::zeros(),

            depth_of_field_disk_u: Vec3::zeros(),
            depth_of_field_disk_v: Vec3::zeros(),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            RenderOutputConfig::default(),
            ViewportConfig::default(),
            LensConfig::default(),
        )
    }
}

impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.origin = self.look_from;

        let theta = self.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Orthonormal Basis
        self.w = (self.look_from - self.look_at).normalize();
        self.u = self.view_up.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        // Viewport Vectors
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.origin - (self.focus_dist * self.w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.0;

        let depth_of_field_radius =
            self.focus_dist * (self.depth_of_field_angle / 2.0).to_radians().tan();
        self.depth_of_field_disk_u = depth_of_field_radius * self.u;
        self.depth_of_field_disk_v = depth_of_field_radius * self.v;
    }

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
                output_image.put_pixel(i, j, pixel_color.to_rgb8());
            }
        }

        output_image
    }

    fn ray_color(&mut self, r: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::zeros();
        }

        let mut rec = HitRecord::default();
        if world.hit(r, Interval::right_open(0.001), &mut rec) {
            let mut scattered = Ray::new(Point3::zeros(), Vec3::zeros(), 0.0);
            let mut attenuation = Color::zeros();

            // let max_intersection_checks = 100.0;
            // rec.mat = Rc::new(Lambertian::from_albedo(Color::new(
            //     (rec.debug.intersection_checks as f64 / max_intersection_checks)
            //         .min(max_intersection_checks),
            //     0.0,
            //     0.0,
            // )));

            // println!("{} intersection checks", rec.debug.intersection_checks);

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

    fn get_ray(&mut self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.depth_of_field_angle > 0.0 {
            self.depth_of_field_disk_sample()
        } else {
            self.origin
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = self.rng.gen::<f64>();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn depth_of_field_disk_sample(&mut self) -> Point3 {
        let p = Vec3::random_unit_disk_vector(&mut self.rng);
        self.origin + (self.depth_of_field_disk_u * p.x()) + (self.depth_of_field_disk_v * p.y())
    }

    fn pixel_sample_square(&mut self) -> Vec3 {
        let px = -0.5 + self.rng.gen::<f64>();
        let py = -0.5 + self.rng.gen::<f64>();

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

            color = color.linear_to_gamma();
            *pixel = color.to_rgb8();
        }
    }
}
