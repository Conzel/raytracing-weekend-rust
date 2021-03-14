use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    origin: Loc,
    horiz: Loc,
    vert: Loc,
    lower_left_corner: Loc,
}

type Degree = f64;

impl Camera {
    pub fn new(lookfrom: Loc, lookat: Loc, vup: Loc, vfov: Degree, aspect_ratio: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&lookfrom - &lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u); // unit vec by definition

        let origin = lookfrom;
        let horiz = viewport_width * u;
        let vert = viewport_height * v;
        let lower_left_corner = &origin - &horiz / 2.0 - &vert / 2.0 - w;

        Camera {
            lower_left_corner: lower_left_corner,
            origin: origin,
            horiz: horiz,
            vert: vert,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            &self.lower_left_corner + s * &self.horiz + t * &self.vert - &self.origin,
        )
    }
}
