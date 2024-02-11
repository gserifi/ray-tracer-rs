use crate::interval::Interval;
pub use crate::vec3::{Color, RGBAccessor};

pub trait ColorWriter {
    fn write_color(&self, samples_per_pixel: i32) -> String;
}

impl ColorWriter for Color {
    fn write_color(&self, samples_per_pixel: i32) -> String {
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
            (256.0 * intensity.clamp(r)) as i32,
            (256.0 * intensity.clamp(g)) as i32,
            (256.0 * intensity.clamp(b)) as i32
        )
    }
}
