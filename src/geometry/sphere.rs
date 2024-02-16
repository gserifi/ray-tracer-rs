use std::rc::Rc;

use crate::geometry::{HitRecord, Hittable};
use crate::materials::{Lambertian, Material};
use crate::optics::Ray;
use crate::utils::{Interval, Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
    center_vec: Vec3,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
            center_vec: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        Self {
            center: center0,
            radius,
            mat,
            center_vec: center1 - center0,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::default()),
        )
    }
}

impl Sphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center + self.center_vec * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        let center = self.center(r.time());
        let oc = r.origin() - center;
        let a = 1.0; // Note that direction vector is normalized
        let half_b = oc.dot(&r.direction());
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Rc::clone(&self.mat);

        true
    }
}
