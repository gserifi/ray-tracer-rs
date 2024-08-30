use crate::textures::Texture;
use crate::utils::{Color, ColorExt, Perlin, Vec3, Vec3Ext};
use rand_distr::num_traits::Pow;

#[derive(Debug)]
pub struct PerlinNoise {
    scale: f64,
    contrast: f64,
    noise: Perlin,
    low_color: Color,
    high_color: Color,
}

impl PerlinNoise {
    pub fn new(scale: f64, contrast: f64, low_color: Color, high_color: Color) -> Self {
        Self {
            scale: 1.0 / scale,
            contrast,
            noise: Perlin::default(),
            low_color,
            high_color,
        }
    }

    pub fn new_bw(scale: f64, contrast: f64) -> Self {
        Self::new(scale, contrast, Color::zeros(), Color::ones())
    }
}

impl Texture for PerlinNoise {
    fn sample(&self, _: f64, _: f64, p: &Vec3) -> Color {
        let factors: Vec3 = Vec3::ones()
            * (0.5 * (1.0 + (20.0 * self.noise.turb(&(self.scale * p), 7)).sin()))
                .pow(self.contrast);

        Color::new(
            self.low_color.r() * (1.0 - factors.x()) + self.high_color.r() * factors.x(),
            self.low_color.g() * (1.0 - factors.y()) + self.high_color.g() * factors.y(),
            self.low_color.b() * (1.0 - factors.z()) + self.high_color.b() * factors.z(),
        )
    }
}
