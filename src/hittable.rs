use crate::ray::*;
use crate::vec3::*;
use crate::materials::Material;

#[derive(Debug, PartialEq, Eq)]
pub enum Surface {
    Inside,
    Outside,
}

#[derive(Debug)]
pub struct Hit<'a> {
    pub location: Loc,
    pub normal: Loc,
    pub t: f64,
    pub material: &'a dyn Material,
    pub surface: Surface,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, validate_t: &dyn Fn(f64) -> bool) -> Option<Hit>;
}

impl<'a> Hit<'a> {
    pub fn new(location: Loc, normal: Loc, t: f64, material: &'a dyn Material, surface: Surface) -> Hit<'a> {
        assert!((normal.length() - 1.0).abs() <= 0.0001);
        Hit {
            location: location,
            normal: normal,
            t: t,
            material: material,
            surface: surface,
        }
    }

    pub fn from_ray(location: Loc, outward_normal: Loc, t: f64, ray: &Ray, material: &'a dyn Material) -> Hit<'a> {
        if ray.dir.dot(&outward_normal) > 0.0 {
            assert!(ray.dir.dot(&-&(outward_normal)) < 0.0);
            Hit::new(location, -outward_normal, t, material, Surface::Inside)
        } else {
            Hit::new(location, outward_normal, t, material, Surface::Outside)
        }
    }
}
