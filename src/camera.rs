use crate::ray::*;
use crate::sampling::*;
use crate::vec3::*;
use rand::thread_rng;
use rand::Rng;

pub struct Camera {
    origin: Loc,
    horiz: Loc,
    vert: Loc,
    lower_left_corner: Loc,
    lens_radius: f64,
    u: Loc,
    v: Loc,
    w: Loc,
}

type Degree = f64;

impl Camera {
    pub fn new(
        lookfrom: Loc,
        lookat: Loc,
        vup: Loc,
        vfov: Degree,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&lookfrom - &lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u); // unit vec by definition

        let origin = lookfrom;
        let horiz = focus_dist * viewport_width * &u;
        let vert = focus_dist * viewport_height * &v;
        let lower_left_corner = &origin - &horiz / 2.0 - &vert / 2.0 - focus_dist * &w;

        Camera {
            lower_left_corner: lower_left_corner,
            origin: origin,
            horiz: horiz,
            vert: vert,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
            w: w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let mut rng = thread_rng();
        let rd = self.lens_radius * Vec3::random_in_unit_disk(&mut rng);
        let offset = &self.u * rd.e0 + &self.v * rd.e1;
        let new_orig = &self.origin + offset;
        let ray_dir = &self.lower_left_corner + s * &self.horiz + t * &self.vert - &new_orig;
        Ray::new(new_orig, ray_dir)
    }
}
