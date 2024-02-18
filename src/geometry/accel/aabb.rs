use crate::optics::Ray;
use crate::utils::{Interval, Point3};

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

pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn wrap_points(a: &Point3, b: &Point3) -> Self {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));
        Self::new(x, y, z)
    }

    pub fn wrap_boxes(box0: &AABB, box1: &AABB) -> Self {
        let x = Interval::wrap_intervals(&box0.x, &box1.x);
        let y = Interval::wrap_intervals(&box0.y, &box1.y);
        let z = Interval::wrap_intervals(&box0.z, &box1.z);
        Self::new(x, y, z)
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self::new(
            Interval::new(0.0, 0.0),
            Interval::new(0.0, 0.0),
            Interval::new(0.0, 0.0),
        )
    }
}

impl AABB {
    pub fn axis(&self, n: &Axis) -> Interval {
        match n {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn hit(&self, ray: &Ray, t: Interval) -> bool {
        for axis in [Axis::X, Axis::Y, Axis::Z].iter() {
            let interval = self.axis(&axis);
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
