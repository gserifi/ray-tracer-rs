extern crate image;
use image::RgbImage;

use crate::textures::Texture;
use crate::utils::{Color, Vec3};

#[derive(Debug)]
pub struct UVImage {
    image: RgbImage,
}

impl UVImage {
    pub fn new(img_path: &str) -> Self {
        let img = image::open(img_path).unwrap().to_rgb8();
        Self { image: img }
    }
}

impl Texture for UVImage {
    fn sample(&self, u: f64, v: f64, _: &Vec3) -> Color {
        let (width, height) = self.image.dimensions();
        let u = u.min(1.0).max(0.0);
        let v = 1.0 - v.min(1.0).max(0.0);
        let i = (u * width as f64) as u32;
        let j = (v * height as f64) as u32;
        let i = i.min(width - 1);
        let j = j.min(height - 1);
        let pixel = self.image.get_pixel(i, j);
        Color::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        )
    }
}
