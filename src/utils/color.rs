pub use crate::utils::{Interval, Vec3, Vec3Ext};
use image::Rgb;

pub type Color = Vec3;

pub trait ColorExt
where
    Self: Vec3Ext,
{
    fn r(&self) -> f64 {
        self.x()
    }

    fn g(&self) -> f64 {
        self.y()
    }

    fn b(&self) -> f64 {
        self.z()
    }

    fn linear_to_gamma(&self) -> Color {
        Color::new(self.r().sqrt(), self.g().sqrt(), self.b().sqrt())
    }

    fn to_rgb8(&self) -> Rgb<u8> {
        let intensity = Interval::new(0.0, 0.999);

        Rgb([
            (256.0 * intensity.clamp(self.r())) as u8,
            (256.0 * intensity.clamp(self.g())) as u8,
            (256.0 * intensity.clamp(self.b())) as u8,
        ])
    }
}

impl ColorExt for Color {}
