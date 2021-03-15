use crate::hittable::*;
use crate::ray::*;

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + Send + Sync + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new(vec: Vec<Box<dyn Hittable + Send + Sync + 'a>>) -> HittableList<'a> {
        HittableList { objects: vec }
    }

    pub fn empty() -> HittableList<'a> {
        HittableList { objects: vec![] }
    }

    pub fn add<T: Hittable + Send + Sync + 'a>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
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
