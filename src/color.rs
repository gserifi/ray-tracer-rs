pub use crate::vec3::Vec3;

pub use Vec3 as Color;

pub fn write_color(pixel_color: Color) -> String {
    let ir: i32 = (255.99 * pixel_color.r()) as i32;
    let ig: i32 = (255.99 * pixel_color.g()) as i32;
    let ib: i32 = (255.99 * pixel_color.b()) as i32;

    format!("{} {} {}", ir, ig, ib)
}
