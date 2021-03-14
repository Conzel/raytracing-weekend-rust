use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;
use rand::thread_rng;
use std::fmt;

pub trait Material: fmt::Debug {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult>;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo: albedo }
    }
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub ray: Ray,
}

impl ScatterResult {
    pub fn new(attenuation: Color, ray: Ray) -> ScatterResult {
        ScatterResult {
            attenuation: attenuation,
            ray: ray,
        }
    }
}

impl Vec3 {
    fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let mut rng = thread_rng();

        let uncorrected_scatter_dir = &hit.normal + Vec3::random_unit_vector(&mut rng);
        // case of scatter direction being exactly opposite
        let scatter_direction = if uncorrected_scatter_dir.is_near_zero() {
            &hit.normal
        } else {
            &uncorrected_scatter_dir
        };

        let scattered_ray = Ray::new(hit.location.clone(), scatter_direction.clone());
        Some(ScatterResult::new(self.albedo.clone(), scattered_ray))
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let reflected_direction = r.unit_direction().reflect(&hit.normal);
        if reflected_direction.dot(&hit.normal) > 0.0 {
            let scattered_ray = Ray::new(hit.location.clone(), reflected_direction);
            Some(ScatterResult::new(self.albedo.clone(), scattered_ray))
        } else {
            None
        }
    }
}
