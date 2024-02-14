use std::rc::Rc;

use crate::materials::{Lambertian, Material};
use crate::optics::Ray;
use crate::utils::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, mat: Rc<dyn Material>, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            mat,
            front_face,
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::zeros(),
            normal: Vec3::zeros(),
            t: 0.0,
            mat: Rc::new(Lambertian::new(Vec3::zeros())),
            front_face: false,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
