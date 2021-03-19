# raytracing-weekend-rust

A Rust-rewrite of [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). Ray tracing is rendering technique in computer graphics, where image generation is done via mathematical models of physical rays, which are originate from an observer and scan the modelled scene. Modern Ray Tracing is able to generate photorealistic images, albeit rather slowly. The aim is to build a functional (albeit minimalistic) ray tracer completely from scratch. This is purely for educational purposes.

Features so far:
- Vector implementation
- Shapes (Spheres)
- Materials (Lambertian, Metal, Glass)
- Shading via Materials
- Moveable Camera 
- Defocus Blur
- Parallelization of rendering via [rayon](https://github.com/rayon-rs/rayon)

As a next goal, I want to implement the follow-up book [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html).

## How to use
The following explanations are for Linux.

1. Follow the Rust installation instructions in [the Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html)
2. Clone this repository
3. Navigate into the repository and build the project using `cargo build`
4. Run the executable and redirect the output to a .ppm file, for example `./target/release/raytracing > image.ppm`
5. View the image using any desired image viewer, feh for example does the trick

Example output (the image on the front of the book cover):
![Book Cover Example](/final.png)
