#[macro_use]
extern crate impl_ops;
use indicatif::ProgressBar;
use rand::Rng;
use string_builder::Builder;
use std::time::Instant;
use rayon::prelude::*;

mod camera;
mod hittable;
mod hittable_list;
mod materials;
mod ray;
mod sampling;
mod sphere;
mod vec3;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 500;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_REC_DEPTH: i32 = 20;
const NUM_SAMPLES: i32 = 10;
const GAMMA_CORRECTION: f64 = 2.0;
const SHADOW_ACNE_TOLERANCE: f64 = 0.0001;
const APERTURE: f64 = 0.1;
const FOCUS_DIST: f64 = 10.0;

fn ray_to_color(
    r: &ray::Ray,
    world: &hittable_list::HittableList,
    recursion_depth: i32,
    rng: &mut impl Rng,
) -> vec3::Color {
    use hittable::Hittable;
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

fn gen_random_scene() -> hittable_list::HittableList<'static> {
    use hittable_list::*;
    use materials::*;
    use vec3::*;
    use sphere::*;

    let mut rng = rand::thread_rng();
    let mut world = HittableList::empty();

    let ground_material = Lambertian::new(Vec3::new(0.5,0.5,0.5));
    let ground_sphere = Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, Box::new(ground_material));

    world.add(ground_sphere);
    for a in -11..11 {
        for b in -11..11 {
            let material_num: f64 = rng.gen();
            let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, 
                                   b as f64 + 0.9 * rng.gen::<f64>());

            if (&center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Box<dyn Material> = if material_num < 0.8 {
                    // diffuse
                    let albedo = Vec3::random_range(0.0, 1.0, &mut rng)
                        .hadamard(&Vec3::random_range(0.0, 1.0, &mut rng));
                    Box::new(Lambertian::new(albedo))
                }
                else if material_num < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0, &mut rng);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Box::new(Metal::new(albedo, fuzz))
                }
                else {
                    // glass
                    Box::new(Dielectric::new(1.5))
                };
                world.add(Sphere::new(center, 0.2, material));
            }
        }
    }
    let material_dielec = Dielectric::new(1.5);
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(material_dielec)));
    let material_lamb = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(material_lamb)));
    let material_metal = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(material_metal)));
    world
}

fn main() {
    use camera::*;
    use sampling::ColorSampler;
    use vec3::*;

    // Camera
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        APERTURE,
        FOCUS_DIST
    );

    // World
    let world = gen_random_scene();


    let bar = ProgressBar::new((IMAGE_HEIGHT).into());
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    
    let now = Instant::now();
    let image_data = (0..IMAGE_HEIGHT).into_par_iter().rev().map(|col| {
        bar.inc(1);
        let mut rng = rand::thread_rng();
        let mut row_output_builder = Builder::default();
        // Anti-Aliasing
        let mut color_sampler = ColorSampler::new();
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

            row_output_builder.append(color_string(&pixel_color));
            row_output_builder.append("\n");
        }
        row_output_builder.string().unwrap()
    }).collect::<Vec<String>>().join("\n");
    bar.finish();

    println!("{}", image_data);
    eprintln!("Finished. Rendering took {} seconds.", now.elapsed().as_secs());
}
