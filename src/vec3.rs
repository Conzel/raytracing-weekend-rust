pub type Color = Vec3;
pub type Loc = Vec3;

use std::ops;
use std::ops::Neg;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    pub e0: f64,
    pub e1: f64,
    pub e2: f64,
}

impl Vec3 {
    pub const fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e0: e0,
            e1: e1,
            e2: e2,
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.e0 * rhs.e0 + self.e1 * rhs.e1 + self.e2 * rhs.e2
    }

    pub fn hadamard(&self, rhs: &Self) -> Vec3 {
        Vec3::new(self.e0 * rhs.e0, self.e1 * rhs.e1, self.e2 * rhs.e2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

pub fn color_string(c: &Color) -> String {
    assert!(valid_color(c));
    format!(
        "{} {} {}",
        (c.e0 * 255.0).round(),
        (c.e1 * 255.0).round(),
        (c.e2 * 255.0).round()
    )
}

fn valid_color(c: &Color) -> bool {
    c.e0 <= 1.0 && c.e0 >= 0.0 && c.e1 <= 1.0 && c.e1 >= 0.0 && c.e2 <= 1.0 && c.e2 >= 0.0
}

impl_op_ex_commutative!(*|v: &Vec3, s: &f64| -> Vec3 { Vec3::new(v.e0 * s, v.e1 * s, v.e2 * s) });

impl_op_ex_commutative!(/ |v: &Vec3, s: &f64| -> Vec3 {
    v * (1.0/s)
});

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(a.e0 + b.e0, a.e1 + b.e1, a.e2 + b.e2)
});

impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { a + (-1.0 * b) });

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        self * (-1.0) 
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        self * (-1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_VEC_1: Vec3 = Vec3::new(1.0,2.0,3.0);
    const TEST_VEC_2: Vec3 = Vec3::new(-1.0,0.0,1.0);

    #[test]
    fn test_add_vectors() {
        let res = Vec3::new(0.0,2.0,4.0);
        assert_eq!(TEST_VEC_1 + TEST_VEC_2, res);
        assert_eq!(TEST_VEC_2 + TEST_VEC_1, res);
    }

    #[test]
    fn test_sub_vectors() {
        assert_eq!(TEST_VEC_1 - TEST_VEC_2, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_mul_scalar() {
        let res = Vec3::new(2.0,4.0,6.0);
        assert_eq!(TEST_VEC_1 * 2.0, res);
        assert_eq!(2.0 * TEST_VEC_1, res);
    }

    #[test]
    fn test_dot_product() {
        assert_eq!(TEST_VEC_1.dot(&TEST_VEC_2), 2.0);
    }

    #[test]
    fn test_lengths() {
        assert_eq!(TEST_VEC_2.length(), (2.0 as f64).sqrt());
        assert_eq!(TEST_VEC_2.length_squared(), 2.0);
    }

    #[test]
    fn test_unit_vector_creation() {
        let sqrt_one_half = 1.0/(2.0 as f64).sqrt();
        assert_eq!(TEST_VEC_2.unit_vector(), Vec3::new(-sqrt_one_half, 0.0, sqrt_one_half));
    }

    #[test]
    fn test_hadamard_product() {
        assert_eq!(TEST_VEC_1.hadamard(&TEST_VEC_2), Vec3::new(-1.0, 0.0, 3.0));
    }

    #[test]
    fn test_color_validation() {
        assert!(!valid_color(&Vec3::new(1.1, 0.0, 1.0)));
        assert!(valid_color(&Vec3::new(0.8, 0.0, 1.0)));
        assert!(!valid_color(&Vec3::new(0.8, 0.0, -0.3)));
    }

    #[test]
    fn test_color_string() {
        assert_eq!("255 255 255", color_string(&Vec3::new(1.0,1.0,1.0)));
    }
}
