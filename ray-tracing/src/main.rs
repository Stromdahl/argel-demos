mod ray;
mod vec3;

use std::time::Instant;
use crate::vec3::Vec3;
use argel::canvas::Canvas;
use argel::output::save_ppm_image;

fn main() {
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = 300;
    const IMAGE_PATH: &str = "image.ppm";
    let mut canvas = Canvas::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("Generating image of size {}x{}...", IMAGE_WIDTH, IMAGE_HEIGHT);
    let now = Instant::now();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
            let pixel_color: u32 = (ir as u32) << 16 | (ig as u32) << 8 | ib as u32;
            canvas.pixels[j * IMAGE_WIDTH + i] = pixel_color;
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
