use std::fmt::Debug;
use std::rc::Rc;

use crate::geometry::Hittable;
use crate::optics::Ray;
use crate::utils::{Interval, Point3, Vec3Ext};

pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn from_idx(idx: usize) -> Self {
        match idx {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => panic!("Invalid axis index"),
        }
    }

    pub fn idx(&self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AABB {
    pub min: Box<Point3>,
    pub max: Box<Point3>,
}

impl AABB {
    fn new(min: Point3, max: Point3) -> Self {
        Self {
            min: Box::new(min),
            max: Box::new(max),
        }
    }

    pub fn wrap_points(a: &Point3, b: &Point3) -> Self {
        let x = Interval::new(a.x().min(b.x()), a.x().max(b.x()));
        let y = Interval::new(a.y().min(b.y()), a.y().max(b.y()));
        let z = Interval::new(a.z().min(b.z()), a.z().max(b.z()));

        Self::new(
            Point3::new(x.min, y.min, z.min),
            Point3::new(x.max, y.max, z.max),
        )
    }

    pub fn wrap_boxes(box0: &AABB, box1: &AABB) -> Self {
        let x = Interval::new(
            box0.min.x().min(box1.min.x()),
            box0.max.x().max(box1.max.x()),
        );
        let y = Interval::new(
            box0.min.y().min(box1.min.y()),
            box0.max.y().max(box1.max.y()),
        );
        let z = Interval::new(
            box0.min.z().min(box1.min.z()),
            box0.max.z().max(box1.max.z()),
        );

        Self::new(
            Point3::new(x.min, y.min, z.min),
            Point3::new(x.max, y.max, z.max),
        )
    }

    pub fn wrap_triangle(vertices: &[Point3; 3]) -> Self {
        let x = Interval::new(
            vertices[0].x().min(vertices[1].x().min(vertices[2].x())),
            vertices[0].x().max(vertices[1].x().max(vertices[2].x())),
        );
        let y = Interval::new(
            vertices[0].y().min(vertices[1].y().min(vertices[2].y())),
            vertices[0].y().max(vertices[1].y().max(vertices[2].y())),
        );
        let z = Interval::new(
            vertices[0].z().min(vertices[1].z().min(vertices[2].z())),
            vertices[0].z().max(vertices[1].z().max(vertices[2].z())),
        );

        Self::new(
            Point3::new(x.min, y.min, z.min),
            Point3::new(x.max, y.max, z.max),
        )
    }

    pub fn wrap_objects(objects: &[Rc<dyn Hittable>]) -> Self {
        let bbox = objects[0].bounding_box();
        let mut bbox = AABB::wrap_points(&bbox.min, &bbox.max);

        for object in objects.iter().skip(1) {
            bbox = AABB::wrap_boxes(&bbox, object.bounding_box());
        }

        bbox
    }
}

impl AABB {
    pub fn axis(&self, n: &Axis) -> Interval {
        match n {
            Axis::X => Interval::new(self.min.x(), self.max.x()),
            Axis::Y => Interval::new(self.min.y(), self.max.y()),
            Axis::Z => Interval::new(self.min.z(), self.max.z()),
        }
    }

    pub fn x(&self) -> Interval {
        self.axis(&Axis::X)
    }

    pub fn y(&self) -> Interval {
        self.axis(&Axis::Y)
    }

    pub fn z(&self) -> Interval {
        self.axis(&Axis::Z)
    }

    pub fn longest_axis(&self) -> Axis {
        let x_len = self.x().size();
        let y_len = self.y().size();
        let z_len = self.z().size();

        if x_len > y_len && x_len > z_len {
            Axis::X
        } else if y_len > z_len {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub fn center(&self) -> Point3 {
        (*self.min + *self.max) / 2.0
    }

    pub fn volume(&self) -> f64 {
        let x = self.x().size();
        let y = self.y().size();
        let z = self.z().size();
        x * y * z
    }

    pub fn surface_area(&self) -> f64 {
        let x = self.x().size();
        let y = self.y().size();
        let z = self.z().size();
        2.0 * (x * y + y * z + z * x)
    }

    pub fn octant(&self, i: usize) -> AABB {
        let x = if i & 1 == 0 {
            self.x().min
        } else {
            self.x().max
        };
        let y = if i & 2 == 0 {
            self.y().min
        } else {
            self.y().max
        };
        let z = if i & 4 == 0 {
            self.z().min
        } else {
            self.z().max
        };
        let corner = Point3::new(x, y, z);
        let center = self.center();
        AABB::wrap_points(&center, &corner)
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.x().intersects(&other.x())
            && self.y().intersects(&other.y())
            && self.z().intersects(&other.z())
    }

    pub fn hit(&self, ray: &Ray, t: Interval) -> bool {
        for axis in [Axis::X, Axis::Y, Axis::Z].iter() {
            let interval = self.axis(axis);
            let inv_d = 1.0 / ray.direction()[axis.idx()];
            let orig = ray.origin()[axis.idx()];

            let mut t0 = (interval.min - orig) * inv_d;
            let mut t1 = (interval.max - orig) * inv_d;

            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }

            if t1.min(t.max) <= t0.max(t.min) {
                return false;
            }
        }

        true
    }
}
