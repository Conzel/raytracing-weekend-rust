#[macro_use]
extern crate impl_ops;
use indicatif::ProgressBar;
use rand::Rng;
mod camera;
mod hittable;
mod hittable_list;
mod materials;
mod ray;
mod sampling;
mod sphere;
mod vec3;
use std::f64::consts;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_REC_DEPTH: i32 = 10;
const NUM_SAMPLES: i32 = 20;
const GAMMA_CORRECTION: f64 = 2.0;
const SHADOW_ACNE_TOLERANCE: f64 = 0.0001;
const REFLECTIVITY: f64 = 0.5;

fn ray_to_color(
    r: &ray::Ray,
    world: &hittable_list::HittableList,
    recursion_depth: i32,
    rng: &mut impl Rng,
) -> vec3::Color {
    use hittable::Hittable;
    use ray::*;
    use sphere::*;
    use vec3::*;

    if recursion_depth <= 0 {
        return Vec3::zero();
    }

    if let Some(hit) = world.hit(r, &interval_validator(Some(SHADOW_ACNE_TOLERANCE), None)) {
        if let Some(scatter_result) = hit.material.scatter(r, &hit) {
            return scatter_result.attenuation.hadamard(&ray_to_color(
                &scatter_result.ray,
                world,
                recursion_depth - 1,
                rng,
            ));
        } else {
            return Vec3::zero();
        }
    }

    let dir = r.unit_direction();
    let t = 0.5 * (dir.e1 + 1.0);
    assert!(
        t >= 0.0 && t <= 1.0,
        "t was: {} ray {:?}, recursion_depth: {}",
        t,
        r,
        recursion_depth
    );
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.2, 0.4, 1.0)
}

fn main() {
    use camera::*;
    use materials::*;
    use sampling::ColorSampler;
    use sphere::*;
    use vec3::*;

    // Camera
    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        200.0,
        16.0 / 9.0,
    );

    // World
    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.7, 0.3, 0.3));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.2);

    let sphere_center = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(material_center));
    let background = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        90.0,
        Box::new(material_ground),
    );
    let sphere_right = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(material_right));
    let sphere_left = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(material_left));
    let world = hittable_list::HittableList::new(vec![
        &sphere_center,
        &background,
        &sphere_left,
        &sphere_right,
    ]);

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
                let sample_color =
                    ray_to_color(&camera.get_ray(u, v), &world, MAX_REC_DEPTH, &mut rng);
                color_sampler.add(&sample_color);
            }
            let pixel_color = color_sampler
                .get_and_reset()
                .gamma_correct(GAMMA_CORRECTION);

            println!("{}", color_string(&pixel_color));
        }
    }
    bar.finish();
}
