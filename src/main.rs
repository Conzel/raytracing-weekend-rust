#[macro_use]
extern crate impl_ops;
use indicatif::ProgressBar;
use std::mem;
use rand::Rng;
mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn ray_to_color(r: &ray::Ray, world: &hittable_list::HittableList) -> vec3::Color {
    use hittable::Hittable;
    use sphere::*;
    use vec3::*;

    if let Some(hit) = world.hit(r, &interval_validator(Some(0.0), None)) {
        let n = hit.normal;
        return 0.5 * (n + Vec3::new(1.0, 1.0, 1.0));
    }

    let dir = r.unit_direction();
    let t = 0.5 * (dir.e1 + 1.0);
    assert!(t >= 0.0 && t <= 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.2, 0.4, 1.0)
}

// We must get both u and v at the same time, as we work with RNG, who can only be borrowed
// mutably.
fn sample(
    num_samples: i32,
    rand_uv: &mut impl FnMut() -> (f64, f64), 
    color_at: &impl Fn(f64, f64) -> vec3::Color,
) -> vec3::Color {
    let mut pixel_color = vec3::Vec3::new(0.0,0.0,0.0);
    for _ in 0..num_samples {
        let (u,v) = rand_uv();
        pixel_color.add_cum(&color_at(u,v));
    }
    pixel_color / (num_samples as f64)
}

pub struct ColorSampler {
    sample_num: i32,
    acc: vec3::Color 
}

impl ColorSampler {
    pub fn new() -> ColorSampler {
        ColorSampler {
            sample_num: 0,
            acc: vec3::Vec3::new(0.0,0.0,0.0)
        }
    }

    pub fn add(&mut self, c: &vec3::Color) {
        self.sample_num += 1;
        self.acc.add_cum(c);
    }

    pub fn get_and_reset(&mut self) -> vec3::Color {
        let mut res = vec3::Vec3::new(0.0,0.0,0.0);
        mem::swap(&mut self.acc, &mut res);
        res = res / (self.sample_num as f64);
        self.sample_num = 0;
        res
    }
}

fn main() {
    use camera::*;
    use sphere::*;
    use vec3::*;

    // Camera
    let camera = Camera::create_simple();

    // World
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let background = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = hittable_list::HittableList::new(vec![&sphere, &background]);

    // Anti-Aliasing
    let mut rng = rand::thread_rng();
    let num_samples = 100;
    let mut color_sampler = ColorSampler::new();

    let bar = ProgressBar::new((IMAGE_HEIGHT).into());
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for col in (0..IMAGE_HEIGHT).rev() {
        bar.inc(1);
        for row in 0..IMAGE_WIDTH {
            for _ in 0..num_samples {
                let u = (row as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                let v = (col as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                let sample_color = ray_to_color(&camera.get_ray(u,v), &world);
                color_sampler.add(&sample_color);
            }
            let pixel_color = color_sampler.get_and_reset();

            println!("{}", color_string(&pixel_color));
        }
    }
    bar.finish();
}
