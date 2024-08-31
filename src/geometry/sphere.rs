use std::f64::consts::PI;
use std::fmt::Debug;
use std::rc::Rc;

use crate::geometry::{accel::AABB, HitRecord, Hittable};
use crate::materials::Material;
use crate::optics::Ray;
use crate::utils::{Interval, Point3, Vec3, Vec3Ext};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
    center_vec: Vec3,
    bbox: AABB,
}

impl Debug for Sphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sphere").finish()
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
            center_vec: Vec3::new(0.0, 0.0, 0.0),
            bbox: AABB::wrap_points(
                &(center - Vec3::constant(radius)),
                &(center + Vec3::constant(radius)),
            ),
        }
    }

    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        let radius_vec = Vec3::constant(radius);
        let bbox0 = AABB::wrap_points(&(center0 - radius_vec), &(center0 + radius_vec));
        let bbox1 = AABB::wrap_points(&(center1 - radius_vec), &(center1 + radius_vec));
        Self {
            center: center0,
            radius,
            mat,
            center_vec: center1 - center0,
            bbox: AABB::wrap_boxes(&bbox0, &bbox1),
        }
    }
}

impl Sphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center + self.center_vec * time
    }

    pub fn uv(&self, p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t, rec) {
            return false;
        }

        rec.debug.inc_intersection_checks();

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
        rec.mat = Rc::clone(&self.mat);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        (rec.u, rec.v) = self.uv(&outward_normal);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
