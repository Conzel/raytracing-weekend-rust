use crate::hittable::*;
use crate::ray::*;

pub struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new(vec: Vec<&'a dyn Hittable>) -> HittableList<'a> {
        HittableList { objects: vec }
    }

    pub fn add(&mut self, obj: &'a impl Hittable) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, validate_t: &dyn Fn(f64) -> bool) -> Option<Hit> {
        let mut closest_hit_opt: Option<Hit> = None;

        for hittable in &self.objects {
            if let Some(hit) = hittable.hit(ray, validate_t) {
                match closest_hit_opt {
                    Some(closest_hit) if closest_hit.t.abs() > hit.t.abs() => {
                        closest_hit_opt = Some(hit)
                    }
                    None => closest_hit_opt = Some(hit),
                    _ => (),
                };
            }
        }
        closest_hit_opt
    }
}
