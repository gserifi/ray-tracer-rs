use std::ops;

pub use Vec3 as Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

// Constructors
impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn one() -> Vec3 {
        Vec3 { e: [1.0, 1.0, 1.0] }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
}

// Accessors
impl Vec3 {
    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn r(&self) -> f64 {
        self[0]
    }

    pub fn g(&self) -> f64 {
        self[1]
    }

    pub fn b(&self) -> f64 {
        self[2]
    }
}

// LinAlg Methods
impl Vec3 {
    pub fn norm2(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }

    fn _dot(v: Vec3, w: Vec3) -> f64 {
        v[0] * w[0] + v[1] * w[1] + v[2] * w[2]
    }

    pub fn dot(&self, w: Vec3) -> f64 {
        Vec3::_dot(*self, w)
    }

    fn _cross(v: Vec3, w: Vec3) -> Vec3 {
        Vec3 {
            e: [
                v[1] * w[2] - v[2] * w[1],
                v[2] * w[0] - v[0] * w[2],
                v[0] * w[1] - v[1] * w[0],
            ],
        }
    }

    pub fn cross(&self, w: Vec3) -> Vec3 {
        Vec3::_cross(*self, w)
    }

    pub fn normalize(v: Vec3) -> Vec3 {
        v / v.norm()
    }

    pub fn normalized(&self) -> Vec3 {
        Vec3::normalize(*self)
    }
}

// Operator overloading

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_zero() {
        let v = Vec3::zero();
        assert_eq!(v, Vec3 { e: [0.0, 0.0, 0.0] });
    }

    #[test]
    fn vec3_one() {
        let v = Vec3::one();
        assert_eq!(v, Vec3 { e: [1.0, 1.0, 1.0] });
    }

    #[test]
    fn vec3_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3 { e: [1.0, 2.0, 3.0] });
    }

    #[test]
    fn vec3_x() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
    }

    #[test]
    fn vec3_y() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.y(), 2.0);
    }

    #[test]
    fn vec3_z() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn vec3_r() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.r(), 1.0);
    }

    #[test]
    fn vec3_g() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.g(), 2.0);
    }

    #[test]
    fn vec3_b() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.b(), 3.0);
    }

    #[test]
    fn vec3_norm2() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.norm2(), 14.0);
    }

    #[test]
    fn vec3_norm() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.norm(), 14.0f64.sqrt());
    }

    #[test]
    fn vec3_dot() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v.dot(w), 6.0);
    }

    #[test]
    fn vec3_cross() {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let w = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v.cross(w), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vec3_normalize() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vec3::normalize(v),
            Vec3::new(
                1.0 / 14.0f64.sqrt(),
                2.0 / 14.0f64.sqrt(),
                3.0 / 14.0f64.sqrt()
            )
        );
    }

    #[test]
    fn vec3_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn vec3_index_mut() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[0] = 4.0;
        v[1] = 5.0;
        v[2] = 6.0;
        assert_eq!(v, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn vec3_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = -v;
        assert_eq!(u, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn vec3_add() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(1.0, 1.0, 1.0);

        let u = v + w;
        assert_eq!(u, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn vec3_sub() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(1.0, 1.0, 1.0);

        let u = v - w;
        assert_eq!(u, Vec3::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn vec3_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = v * 2.0;

        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = 2.0 * v;

        assert_eq!(u, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(w, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vec3_div() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = v / 2.0;

        assert_eq!(u, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn vec3_add_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(1.0, 1.0, 1.0);

        v += w;
        assert_eq!(v, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn vec3_sub_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(1.0, 1.0, 1.0);

        v -= w;
        assert_eq!(v, Vec3::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn vec3_mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vec3_div_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, Vec3::new(0.5, 1.0, 1.5));
    }
}
