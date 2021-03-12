use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    origin: Loc,
    horiz: Loc,
    vert: Loc,
    lower_left_corner: Loc
}

impl Camera {
    pub fn create_parallel(aspect_ratio: f64, viewport_height: f64, focal_length: f64, origin: Loc) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let horiz = Vec3::new(viewport_width, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_height, 0.0);
        Camera {
            lower_left_corner: &origin - &horiz / 2.0 - &vert / 2.0 - Vec3::new(0.0, 0.0, focal_length),
            origin: origin,
            horiz: horiz,
            vert: vert,
        }
    }
    
    pub fn create_simple() -> Camera {
        Camera::create_parallel(16.0/9.0, 2.0, 1.0, Vec3::new(0.0,0.0,0.0))
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin.clone(), &self.lower_left_corner + u*&self.horiz + v*&self.vert - &self.origin)
    }
}
