use std::rc::Rc;

use crate::geometry::{HitRecord, Hittable};
use crate::materials::{Lambertian, Material};
use crate::optics::Ray;
use crate::utils::{Interval, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
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

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Rc::clone(&self.mat);

        true
    }
}
