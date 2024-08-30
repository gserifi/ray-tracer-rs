use std::rc::Rc;

use crate::materials::{Lambertian, Material};
use crate::optics::Ray;
use crate::utils::Vec3;

#[derive(Clone, Default)]
pub struct HitRecordDebug {
    pub intersection_checks: u32,
}

impl HitRecordDebug {
    pub fn inc_intersection_checks(&mut self) {
        self.intersection_checks += 1;
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub debug: HitRecordDebug,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::zeros(),
            normal: Vec3::zeros(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            mat: Rc::new(Lambertian::from_albedo(Vec3::zeros())),
            front_face: false,
            debug: HitRecordDebug::default(),
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
