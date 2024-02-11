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
