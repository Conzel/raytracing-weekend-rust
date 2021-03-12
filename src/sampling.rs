use std::mem;
use crate::vec3::*;

pub struct ColorSampler {
    sample_num: i32,
    acc: Color 
}

impl ColorSampler {
    pub fn new() -> ColorSampler {
        ColorSampler {
            sample_num: 0,
            acc: Vec3::new(0.0,0.0,0.0)
        }
    }

    pub fn add(&mut self, c: &Color) {
        self.sample_num += 1;
        self.acc.add_cum(c);
    }

    pub fn get_and_reset(&mut self) -> Color {
        let mut res = Vec3::new(0.0,0.0,0.0);
        mem::swap(&mut self.acc, &mut res);
        res = res / (self.sample_num as f64);
        self.sample_num = 0;
        res
    }
}
