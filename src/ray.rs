use crate::vec3::*;

pub struct Ray {
    pub orig: Loc,
    pub dir: Loc
}

impl Ray {
    pub fn new(orig: Loc, dir: Loc) -> Ray {
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
