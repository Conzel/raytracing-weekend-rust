#[macro_use]
extern crate impl_ops;
use indicatif::ProgressBar;
mod vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    let bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT).into());
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for row in 0..IMAGE_WIDTH {
        for col in 0..IMAGE_HEIGHT {
            bar.inc(1);
            let r = (row as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (col as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25 as f64;

            let ir = (r * 255.0).round();
            let ig = (g * 255.0).round();
            let ib = (b * 255.0).round();

            println!("{} {} {}", ir, ig, ib);
        }
    }
    bar.finish();
}
