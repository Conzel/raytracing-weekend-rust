use crate::vec3::*;
use crate::ray::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Surface {Inside, Outside}

#[derive(Debug)]
pub struct Hit {
    pub location: Loc,
    pub normal: Loc,
    pub t: f64,
    pub surface: Surface
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, validate_t: &dyn Fn(f64) -> bool) -> Option<Hit>;
}

impl Hit {
    pub fn new (location: Loc, normal: Loc, t: f64, surface: Surface) -> Hit {
        assert!((normal.length() - 1.0).abs() <= 0.0001);
        Hit {
            location: location,
            normal: normal,
            t: t,
            surface: surface
        }
    }
    
    pub fn from_ray(location: Loc, outward_normal: Loc, t: f64, ray: &Ray) -> Hit {
        if ray.dir.dot(&outward_normal) > 0.0 {
            assert!(ray.dir.dot(&-&(outward_normal)) < 0.0);
            Hit::new(location, -outward_normal, t, Surface::Inside)
        }
        else {
            Hit::new(location, outward_normal, t, Surface::Outside)
        }
    }
}

