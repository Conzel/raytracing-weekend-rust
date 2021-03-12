use crate::hittable::*;
use crate::materials::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Debug)]
pub struct Sphere {
    pub center: Loc,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Loc, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

pub fn trivial_validator() -> impl Fn(f64) -> bool {
    |_| true
}

pub fn interval_validator(a_opt: Option<f64>, b_opt: Option<f64>) -> impl Fn(f64) -> bool {
    move |t| {
        let a_ok = match a_opt {
            Some(a) => a <= t,
            None => true,
        };
        let b_ok = match b_opt {
            Some(b) => b >= t,
            None => true,
        };
        a_ok && b_ok
    }
}

fn solve_pq(p: f64, q: f64) -> Option<(f64, f64)> {
    let p_half_sq = (p / 2.0).powf(2.0);

    if p_half_sq < q {
        return None;
    }

    let root = (p_half_sq - q).sqrt();
    Some((-p / 2.0 - root, -p / 2.0 + root))
}

fn first_acceptable<T: Clone>(vec: Vec<T>, validate: &dyn Fn(T) -> bool) -> Option<T> {
    for el in vec {
        if validate(el.clone()) {
            return Some(el);
        }
    }
    None
}

impl Hittable for Sphere {
    #[allow(non_snake_case)]
    fn hit(&self, ray: &Ray, validate_t: &dyn Fn(f64) -> bool) -> Option<Hit> {
        // Equation:
        // Ray is described via A + t*b (A origin, b direction), for t in (-oo, oo)
        // Sphere is described via C (center) and r (radius).
        // We can describe whether the sphere is hit by the ray via finding a solution for
        // t^2 <b,b> + 2t <b, (A - C)>  + <A - C, A - C> - r^2 = 0
        // We have a solution if (p/2)^2 >= q (solved above quadratic equation via pq-formula)
        // with p = <b, A - C> / <b,b> and q = <A - C, A - C> - r^2 / <b,b>

        // Solve Equation for roots
        let A = &ray.orig;
        let b = &ray.dir;
        let A_C = A - &self.center;

        let bb = b.dot(b);
        let p = 2.0 * b.dot(&A_C) / bb;
        let q = (A_C.dot(&A_C) - self.radius.powf(2.0)) / bb;

        let (t1, t2) = solve_pq(p, q)?;
        // Check if any roots fulfill validation criteria and return them in case
        let t = first_acceptable(vec![t1, t2], validate_t)?;

        let hit_location = ray.at(t);
        let outward_normal = (&hit_location - &self.center).unit_vector();
        Some(Hit::from_ray(
            hit_location,
            outward_normal,
            t,
            ray,
            &*self.material,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable() {
        let trivial_true = |_: f64| true;
        let trivial_false = |_: f64| false;
        let vec = vec![1.0, 2.0, 3.0];
        assert_eq!(first_acceptable(vec.clone(), &trivial_true), Some(1.0));
        assert_eq!(first_acceptable(vec, &trivial_false), None);
    }

    #[test]
    fn test_pq_solver() {
        let pq_sol_1 = solve_pq(4.0, -5.0);
        assert!(!pq_sol_1.is_none());
        let (x1, x2) = pq_sol_1.unwrap();
        assert_eq!(x1, -5.0);
        assert_eq!(x2, 1.0);
        let pq_sol_2 = solve_pq(0.0, 1.0);
        assert_eq!(pq_sol_2, None);
    }

    #[test]
    fn test_validators() {
        assert!(trivial_validator()(2.0));
        assert!(trivial_validator()(-1.0));
        let interval_1 = interval_validator(Some(0.0), Some(1.0));
        let interval_2 = interval_validator(None, Some(1.0));
        let interval_3 = interval_validator(Some(0.0), None);
        assert!(interval_1(0.5));
        assert!(!interval_1(1.2));
        assert!(!interval_1(-0.7));

        assert!(interval_2(0.5));
        assert!(!interval_2(1.2));
        assert!(interval_2(-0.7));

        assert!(interval_3(0.5));
        assert!(interval_3(1.2));
        assert!(!interval_3(-0.7));
    }

    #[test]
    fn sphere_hits() {
        // Sphere hit from inside
        let validator = interval_validator(Some(0.0), None);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let ray1 = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let hit1 = sphere
            .hit(&ray1, &validator)
            .expect("Ray 1 should've hit sphere");
        assert_eq!(hit1.t, 1.0);
        assert_eq!(hit1.normal, Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(hit1.location, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(hit1.surface, Surface::Inside);

        // Sphere hit from outside
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let ray2 = Ray::new(Vec3::new(0.0, 3.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit2 = sphere
            .hit(&ray2, &validator)
            .expect("Ray 2 should've hit sphere");
        assert_eq!(hit2.t, 2.0);
        assert_eq!(hit2.normal, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(hit2.location, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(hit2.surface, Surface::Outside);

        // Sphere not hit
        let ray3 = Ray::new(Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let hit3 = sphere.hit(&ray3, &validator);
        assert!(hit3.is_none());

        // Sphere cancelled by validator
        let ray4 = Ray::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let hit4 = sphere.hit(&ray4, &validator);
        assert!(hit4.is_none());
    }
}
