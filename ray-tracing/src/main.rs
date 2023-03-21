mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use argel::canvas::Canvas;
use argel::output::save_ppm_image;
use hittable::{Hittable, World};
use rand::Rng;
use std::time::Instant;
use vec3::{Color, Point3};

fn format_color(color: Color, samples_per_pixel: u64) -> u32 {
    let ir = (256.0 * (color.x() / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u8;
    let ig = (256.0 * (color.y() / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u8;
    let ib = (256.0 * (color.z() / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u8;
    (ir as u32) << 16 | (ig as u32) << 8 | ib as u32
}

fn ray_color(r: &ray::Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, std::f64::INFINITY) {
        return 0.5 * rec.normal + Color::new(1.0, 1.0, 1.0);
    }
    let unit_direction = vec3::Vec3::normalized(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    const IMAGE_PATH: &str = "image.ppm";

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;

    const SAMPLES_PER_PIXEL: u64 = 10;

    // World
    let mut world = World::new();
    world.push(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.push(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let camera = camera::Camera::new();

    println!(
        "Generating image of size {}x{}...",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );

    let mut rng = rand::thread_rng();
    let mut canvas = Canvas::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let now = Instant::now();
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            // Sample pixel
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world);
            }

            canvas.pixels[j * IMAGE_WIDTH + i] = format_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    let elapsed = now.elapsed();
    println!(
        "Done! Took {}ms",
        (elapsed.as_nanos() / 1_000_0) as f64 / 100.0
    );
    println!("Saving image to '{}'", IMAGE_PATH);
    save_ppm_image(canvas.pixels, IMAGE_WIDTH, IMAGE_HEIGHT, IMAGE_PATH).unwrap();
    println!("Done!");
}
