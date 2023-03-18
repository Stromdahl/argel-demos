mod ray;
mod vec3;

use std::time::Instant;
use argel::canvas::Canvas;
use argel::output::save_ppm_image;
use vec3::Color;

fn vec3_to_color(color: Color) -> u32 {
    let ir = (255.999 * color.x()) as u8;
    let ig = (255.999 * color.y()) as u8;
    let ib = (255.999 * color.z()) as u8;
    (ir as u32) << 16 | (ig as u32) << 8 | ib as u32
}

fn ray_color(r: &ray::Ray) -> Color {
    let unit_direction = vec3::Vec3::normalized(*r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const IMAGE_PATH: &str = "image.ppm";
    let mut canvas = Canvas::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new(0.0, 0.0, focal_length);

    println!("Generating image of size {}x{}...", IMAGE_WIDTH, IMAGE_HEIGHT);
    let now = Instant::now();

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = ray::Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(&r);
            canvas.pixels[j * IMAGE_WIDTH + i] = vec3_to_color(color);
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
