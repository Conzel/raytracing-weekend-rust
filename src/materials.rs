use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;
use rand::thread_rng;
use rand::Rng;
use std::fmt;

pub trait Material: fmt::Debug + Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzziness: fuzziness,
        }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric {
            refractive_index: refractive_index,
        }
    }

    fn reflectance(cosine: f64, eta_ratio: f64) -> f64 {
        let r0 = ((1.0 - eta_ratio) / (1.0 + eta_ratio)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }

    // Eta eta_ratio: eta / eta'
    // Assumes self and normal to be unit vectors
    // Returns None if no refraction is possible (total reflection)
    #[allow(non_snake_case)]
    fn refract(&self, incoming: &Vec3, normal: &Vec3, eta_ratio: f64) -> Option<Vec3> {
        let mut rng = thread_rng();
        let cos_theta = incoming.dot(&-normal).min(1.0);

        if Self::reflectance(cos_theta, eta_ratio) > rng.gen::<f64>() {
            return None;
        }

        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        if eta_ratio * sin_theta <= 1.0 {
            let R_orth_prime = eta_ratio * (normal - cos_theta * normal);
            let R_par_prime = -(1.0 - R_orth_prime.length_squared()).sqrt() * normal;
            Some(R_orth_prime + R_par_prime)
        } else {
            None
        }
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
    // Assumes self and normal to be unit vectors
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
        let mut rng = thread_rng();

        let reflected_direction = r.unit_direction().reflect(&hit.normal)
            + self.fuzziness * Vec3::random_in_unit_sphere(&mut rng);
        if reflected_direction.dot(&hit.normal) > 0.0 {
            let scattered_ray = Ray::new(hit.location.clone(), reflected_direction);
            Some(ScatterResult::new(self.albedo.clone(), scattered_ray))
        } else {
            // surface absorb
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let eta_frac = if hit.surface == Surface::Inside {
            self.refractive_index
        } else {
            1.0 / self.refractive_index
        };

        let direction = if let Some(refracted_direction) =
            self.refract(&r.unit_direction(), &hit.normal, eta_frac)
        {
            refracted_direction // refraction case
        } else {
            r.unit_direction().reflect(&hit.normal) // total reflection case
        };
        Some(ScatterResult::new(
            attenuation,
            Ray::new(hit.location.clone(), direction),
        ))
    }
}
