use crate::vec3::*;

pub struct Ray {
    pub orig: Loc,
    pub dir: Loc
}

impl Ray {
    pub const fn new(orig: Loc, dir: Loc) -> Ray {
    Ray {
        orig: orig, 
        dir: dir
    }
    }
    pub fn at(&self, t: f64) -> Loc {
        &self.orig + t * &self.dir
    }

    pub fn unit_direction(&self) -> Loc {
        self.dir.unit_vector()
    }
}

#[cfg(tests)]
mod tests {
    use super::*;
    const RAY: Ray = Ray::new(Vec3::new(1.0,0.0,-1.0), Vec3::new(-1.0,0.0,1.0));

    #[test]
    fn test_at() {
        assert_eq!(RAY.at(1.0), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_unit_direction() {
        let sqrt_one_half = 1.0/(2.0 as f64).sqrt();
        assert_eq!(RAY.unit_direction(), Vec3::new(-sqrt_one_half, 0.0, sqrt_one_half));
    }
}
