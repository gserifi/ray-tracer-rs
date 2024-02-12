extern crate nalgebra as na;

use na::Vector3 as _Vector3;

pub type Vec3 = _Vector3<f64>;
pub type Point3 = _Vector3<f64>;
pub type Color = _Vector3<f64>;

pub trait XYZAccessor {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

pub trait RGBAccessor {
    fn r(&self) -> f64;
    fn g(&self) -> f64;
    fn b(&self) -> f64;
}

impl XYZAccessor for Vec3 {
    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn z(&self) -> f64 {
        self[2]
    }
}

impl RGBAccessor for Color {
    fn r(&self) -> f64 {
        self[0]
    }

    fn g(&self) -> f64 {
        self[1]
    }

    fn b(&self) -> f64 {
        self[2]
    }
}

pub fn near_zero(v: &Vec3) -> bool {
    let s = 1e-8;
    v.x().abs() < s && v.y().abs() < s && v.z().abs() < s
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
