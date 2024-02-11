pub use crate::vec3::{Color, RGBAccessor};

pub trait ColorWriter {
    fn write_color(&self) -> String;
}

impl ColorWriter for Color {
    fn write_color(&self) -> String {
        let ir: i32 = (255.99 * self.r()) as i32;
        let ig: i32 = (255.99 * self.g()) as i32;
        let ib: i32 = (255.99 * self.b()) as i32;

        format!("{} {} {}", ir, ig, ib)
    }
}
