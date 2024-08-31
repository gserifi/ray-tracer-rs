use crate::geometry::{accel::AABB, HitRecord, Hittable};
use crate::materials::Material;
use crate::optics::Ray;
use crate::utils::{Interval, Point3, Vec3};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / n.norm_squared();

        let bbox_diag1 = AABB::wrap_points(&q, &(q + u + v));
        let bbox_diag2 = AABB::wrap_points(&(q + u), &(q + v));
        let bbox = AABB::wrap_boxes(&bbox_diag1, &bbox_diag2);

        Self {
            q,
            u,
            v,
            w,
            normal,
            d,
            mat,
            bbox,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&r.direction());
        if denom.abs() < 1e-8 {
            return false;
        }

        let t_int = (self.d - self.normal.dot(&r.origin())) / denom;
        if !t.surrounds(t_int) {
            return false;
        }

        let p = r.at(t_int);
        let planar_hitpt_vec = p - self.q;

        let alpha = self.w.dot(&(planar_hitpt_vec.cross(&self.v)));
        let beta = self.w.dot(&(self.u.cross(&planar_hitpt_vec)));

        if !(0.0 <= alpha && alpha <= 1.0 && 0.0 <= beta && beta <= 1.0) {
            return false;
        }

        rec.t = t_int;
        rec.p = p;
        rec.mat = Rc::clone(&self.mat);
        rec.set_face_normal(r, self.normal);
        (rec.u, rec.v) = (alpha, beta);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
