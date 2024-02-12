use crate::interval::Interval;
pub use crate::vec3::{Color, RGBAccessor};
use image::Rgb;

pub trait ColorWriter {
    fn write_color(&self, samples_per_pixel: u32) -> String;
    fn to_rgb(&self) -> Rgb<u8>;
}

impl ColorWriter for Color {
    fn write_color(&self, samples_per_pixel: u32) -> String {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        let intensity = Interval::new(0.0, 0.999);

        format!(
            "{} {} {}",
            (256.0 * intensity.clamp(r)) as u32,
            (256.0 * intensity.clamp(g)) as u32,
            (256.0 * intensity.clamp(b)) as u32
        )
    }

    fn to_rgb(&self) -> Rgb<u8> {
        let intensity = Interval::new(0.0, 0.999);

        Rgb([
            (256.0 * intensity.clamp(self.r())) as u8,
            (256.0 * intensity.clamp(self.g())) as u8,
            (256.0 * intensity.clamp(self.b())) as u8,
        ])
    }
}
