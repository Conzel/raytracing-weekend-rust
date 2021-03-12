#[macro_use]
extern crate impl_ops;
use indicatif::ProgressBar;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn ray_to_color(r: &ray::Ray, world: &hittable_list::HittableList) -> vec3::Color {
    use vec3::*;
    use sphere::*;
    use hittable::Hittable;

    if let Some(hit) = world.hit(r, &interval_validator(Some(0.0), None)) {
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
    use sphere::*;
    use camera::*;

    // Camera
    let camera = Camera::create_simple();

    // World
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let background = Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0);
    let world = hittable_list::HittableList::new(vec![&sphere, &background]);

    let bar = ProgressBar::new((IMAGE_HEIGHT).into());
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for col in (0..IMAGE_HEIGHT).rev() {
        bar.inc(1);
        for row in 0..IMAGE_WIDTH {
            let u = (row as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (col as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let r = camera.get_ray(u, v);

            let c: Color = ray_to_color(&r, &world);

            println!("{}", color_string(&c));
        }
    }
    bar.finish();
}
