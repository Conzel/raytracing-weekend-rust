#[macro_use]
extern crate impl_ops;
use indicatif::ProgressBar;
mod vec3;
mod ray;
mod sphere;
mod hittable;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const ORIGIN: vec3::Loc = vec3::Vec3::new(0.0,0.0,0.0);
const HORIZ: vec3::Loc = vec3::Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VERT: vec3::Loc = vec3::Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

fn ray_to_color(r: &ray::Ray) -> vec3::Color {
    use vec3::*;
    use sphere::*;
    use hittable::Hittable;

    let center = Vec3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    let sphere = Sphere::new(center,radius);
    if let Some(hit) = sphere.hit(r, &interval_validator(Some(0.0), None)) {
        let n = hit.normal;
        return 0.5 * (n + Vec3::new(1.0,1.0,1.0));
    }

    let dir = r.unit_direction();
    let t = 0.5 * (dir.e1 + 1.0);
    assert!(t >= 0.0 && t <= 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.2,0.4,1.0)
}

fn main() {
    use vec3::*;
    use ray::*;

    let lower_left: vec3::Loc = ORIGIN - HORIZ / 2.0 - VERT / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let bar = ProgressBar::new((IMAGE_HEIGHT).into());
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for col in 0..IMAGE_HEIGHT {
        bar.inc(1);
        for row in 0..IMAGE_WIDTH {
            let u = (row as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (col as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let r = Ray::new(ORIGIN, &HORIZ * u + &VERT * v + &lower_left - ORIGIN);

            let c: Color = ray_to_color(&r);

            println!("{}", color_string(&c));
        }
    }
    bar.finish();
}
