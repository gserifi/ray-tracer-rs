use std::fmt::Debug;

use crate::utils::{Color, Vec3};

pub trait Texture: Debug {
    fn sample(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
