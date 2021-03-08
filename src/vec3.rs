
pub mod vec3 {
    use std::ops;
    #[derive(Debug)]
    pub struct Vec3 {
        pub e0: f64,
        pub e1: f64,
        pub e2: f64,
    }

    impl Vec3 {
        pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
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
    }

    impl_op_ex_commutative!(* |v: &Vec3, s: &f64| -> Vec3 {
        Vec3::new(v.e0 * s, v.e1 * s, v.e2 * s)
    });

    impl_op_ex_commutative!(/ |v: &Vec3, s: &f64| -> Vec3 {
        v * (1.0/s)
    });

    impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
        Vec3::new(a.e0 + b.e0, a.e1 + b.e1, a.e2 + b.e2)
    });

    impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 {
        a + (-1.0 * b)
    });
}
