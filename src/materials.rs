use std::fmt;

pub trait Material: fmt::Debug {
}

#[derive(Debug)]
pub struct Lambertian {}

impl Lambertian {
    pub fn new() -> Lambertian {
    Lambertian{}
    }
}

impl Material for Lambertian {}
