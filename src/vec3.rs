use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    pub e0: f64,
    pub e1: f64,
    pub e2: f64
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e0: e0,
            e1: e1,
            e2: e2
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.e0 * rhs.e0 + self.e1 * rhs.e1 + self.e2 * rhs.e2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.e0 + rhs.e0, self.e1 + rhs.e1, self.e2 + rhs.e2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vec3::new(self.e0 * scalar, self.e1 * scalar, self.e2 * scalar)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (rhs * -1.0)
    }
}

