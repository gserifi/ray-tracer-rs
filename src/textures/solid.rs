use crate::textures::Texture;
use crate::utils::{Color, Vec3};

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
    fn sample(&self, _: f64, _: f64, _: &Vec3) -> Color {
        self.color
    }
}
