use crate::utils::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(), // Keeping the direction vector at unit length
            time,
        }
    }
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
