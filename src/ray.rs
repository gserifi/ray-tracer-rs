use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

// Constructors
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
}

// Accessors
impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

// Methods
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_origin() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(r.origin(), Point3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn ray_direction() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(r.direction(), Point3::new(1.0, 1.0, 1.0).normalize());
    }

    #[test]
    fn ray_at() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(r.at(0.0), Point3::new(1.0, 2.0, 3.0));
        assert_eq!(r.at(1.0), r.origin() + r.direction());
    }
}
