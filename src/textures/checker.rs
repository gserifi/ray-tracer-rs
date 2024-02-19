use crate::textures::{Solid, Texture};
use crate::utils::{Color, Vec3, Vec3Ext};
use std::rc::Rc;

#[derive(Debug)]
pub struct XYZChecker {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl XYZChecker {
    pub fn from_textures(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, even: Color, odd: Color) -> Self {
        Self::from_textures(scale, Rc::new(Solid::new(even)), Rc::new(Solid::new(odd)))
    }
}

impl Texture for XYZChecker {
    fn sample(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.sample(u, v, p)
        } else {
            self.odd.sample(u, v, p)
        }
    }
}

#[derive(Debug)]
pub struct UVChecker {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl UVChecker {
    pub fn from_textures(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, even: Color, odd: Color) -> Self {
        Self::from_textures(scale, Rc::new(Solid::new(even)), Rc::new(Solid::new(odd)))
    }
}

impl Texture for UVChecker {
    fn sample(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let x = (self.inv_scale * u).floor() as i32;
        let y = (self.inv_scale * v).floor() as i32;

        let is_even = (x + y) % 2 == 0;

        if is_even {
            self.even.sample(u, v, p)
        } else {
            self.odd.sample(u, v, p)
        }
    }
}
