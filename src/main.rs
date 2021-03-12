#[macro_use]
extern crate impl_ops;
use indicatif::ProgressBar;
use rand::Rng;
mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod sampling;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_REC_DEPTH: i32 = 15;
const NUM_SAMPLES: i32 = 20;
const GAMMA_CORRECTION: f64 = 2.0;
const SHADOW_ACNE_TOLERANCE: f64 = 0.0001;
const REFLECTIVITY: f64 = 0.5;

fn ray_to_color(r: &ray::Ray, world: &hittable_list::HittableList, recursion_depth: i32, rng: &mut impl Rng) -> vec3::Color {
    use hittable::Hittable;
    use sphere::*;
    use vec3::*;
    use ray::*;

    if recursion_depth <= 0 {
        return Vec3::zero();
    }

    if let Some(hit) = world.hit(r, &interval_validator(Some(SHADOW_ACNE_TOLERANCE), None)) {
        let next_ray_target = &hit.location + &hit.normal + Vec3::random_unit_vector(rng);
        return REFLECTIVITY * ray_to_color(&Ray::new(hit.location, next_ray_target), world, recursion_depth - 1, rng);
    }

    let dir = r.unit_direction();
    let t = 0.5 * (dir.e1 + 1.0);
    assert!(t >= 0.0 && t <= 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.2, 0.4, 1.0)
}

fn main() {
    use camera::*;
    use sphere::*;
    use vec3::*;
    use sampling::ColorSampler;

    // Camera
    let camera = Camera::create_simple();

    // World
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let background = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = hittable_list::HittableList::new(vec![&sphere, &background]);

    // Anti-Aliasing
    let mut rng = rand::thread_rng();
    let mut color_sampler = ColorSampler::new();

    let bar = ProgressBar::new((IMAGE_HEIGHT).into());
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for col in (0..IMAGE_HEIGHT).rev() {
        bar.inc(1);
        for row in 0..IMAGE_WIDTH {
            for _ in 0..NUM_SAMPLES {
                let u = (row as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                let v = (col as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                let sample_color = ray_to_color(&camera.get_ray(u,v), &world, MAX_REC_DEPTH, &mut rng);
                color_sampler.add(&sample_color);
            }
            let pixel_color = color_sampler.get_and_reset().gamma_correct(GAMMA_CORRECTION);

            println!("{}", color_string(&pixel_color));
        }
    }
    bar.finish();
}
