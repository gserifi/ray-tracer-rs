use crate::interval::Interval;
pub use crate::vec3::{Color, RGBAccessor};
use image::Rgb;

pub fn linear_to_gamma(linear_component: &Color) -> Color {
    Color::new(
        linear_component.r().sqrt(),
        linear_component.g().sqrt(),
        linear_component.b().sqrt(),
    )
}

pub fn color_to_rgb8(color: &Color) -> Rgb<u8> {
    let intensity = Interval::new(0.0, 0.999);

    Rgb([
        (256.0 * intensity.clamp(color.r())) as u8,
        (256.0 * intensity.clamp(color.g())) as u8,
        (256.0 * intensity.clamp(color.b())) as u8,
    ])
}
