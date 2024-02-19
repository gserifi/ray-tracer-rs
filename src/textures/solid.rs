use crate::textures::Texture;
use crate::utils::Color;

#[derive(Debug)]
pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for Solid {
    fn sample(&self, _: f64, _: f64, _: &crate::utils::Vec3) -> Color {
        self.color
    }
}
