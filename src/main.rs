mod color;
mod hittable;
mod ray;
mod vec3;

use crate::color::ColorWriter;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3, XYZAccessor};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = 1.0;
    let half_b = oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn write_ppm(w: i32, h: i32) {
    println!("P3\n{} {}\n{}", w, h, 255);

    for j in (0..h).rev() {
        for i in 0..w {
            let col = Color::new(i as f64 / w as f64, j as f64 / h as f64, 0.1);
            println!("{}", col.write_color());
        }
    }
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 720;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    // Camera

    let focal_length = 1.0;
    let camera_origin = Vec3::zeros();

    // Viewport Vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_origin - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    println!("P3\n{} {}\n{}", image_width, image_height, 255);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_origin;
            let r = Ray::new(camera_origin, ray_direction);

            let pixel_color = ray_color(&r);
            println!("{}", pixel_color.write_color());
        }
    }
}
