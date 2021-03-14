use crate::vec3::*;
use rand::Rng;
use std::mem;

pub struct ColorSampler {
    sample_num: i32,
    acc: Color,
}

impl ColorSampler {
    pub fn new() -> ColorSampler {
        ColorSampler {
            sample_num: 0,
            acc: Vec3::zero(),
        }
    }

    pub fn add(&mut self, c: &Color) {
        self.sample_num += 1;
        self.acc.add_cum(c);
    }

    pub fn get_and_reset(&mut self) -> Color {
        let mut res = Vec3::zero();
        mem::swap(&mut self.acc, &mut res);
        res = res / (self.sample_num as f64);
        self.sample_num = 0;
        res
    }
}

// Returns random float in [-1,1]
fn rand_coord(rng: &mut impl Rng) -> f64 {
    (rng.gen::<f64>() - 1.0 / 2.0) * 2.0
}

impl Vec3 {
    pub fn random_in_unit_cube(rng: &mut impl Rng) -> Vec3 {
        Vec3::new(rand_coord(rng), rand_coord(rng), rand_coord(rng))
    }

    // Via rejection sampling
    pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
        let mut candidate = Vec3::random_in_unit_cube(rng);
        while candidate.length_squared() >= 1.0 {
            candidate = Vec3::random_in_unit_cube(rng);
        }
        candidate
    }

    pub fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
        Vec3::random_in_unit_sphere(rng).unit_vector()
    }

    pub fn random_in_hemisphere(normal: Vec3, rng: &mut impl Rng) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(&normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    // Returns random vector in disk with 0 depth z
    pub fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
        let mut candidate = Vec3::new(rand_coord(rng), rand_coord(rng), 0.0);
        while candidate.length_squared() >= 1.0 {
            candidate = Vec3::random_in_unit_cube(rng);
        }
        candidate
    }
}
