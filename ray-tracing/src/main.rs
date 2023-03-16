mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;
use argel::canvas::Canvas;
use argel::color;
use argel::output;

fn ray_color(r: &Ray) -> u32 {
    let unit_direction = Vec3::unit(r.direction().clone());
    let t = 0.5 * (unit_direction.y + 1.0);
    let color1 = Vec3::new(1.0, 1.0, 1.0);
    let color2 = Vec3::new(0.5, 0.7, 1.0);
    let color = (1.0 - t) * color1 + t * color2;
    color::color(
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8,
    )
}

fn main() {
    // // Image
    const APECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / APECT_RATIO) as usize;

    // // Camera
    // let viewport_height: f32 = 2.0;
    // let viewport_width: f32 = APECT_RATIO * viewport_height;
    // let focal_length = 1.0;

    // let origin = Vec3::new(0.0, 0.0, 0.0);
    // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // let vertiacal = Vec3::new(0.0, viewport_height, 0.0);
    // let lower_left_corner =
    //     origin - horizontal / 2.0 - vertiacal / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // //Render
    // let mut canvas: Canvas = Canvas::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // for j in 0..IMAGE_HEIGHT {
    //     for i in 0..IMAGE_WIDTH {
    //         let u = (i / (IMAGE_WIDTH - 1)) as f32;
    //         let v = (j / (IMAGE_HEIGHT - 1)) as f32;
    //         let r: Ray = Ray::new(
    //             origin,
    //             lower_left_corner + u * horizontal + v * vertiacal - origin,
    //         );
    //         let pixel_color = ray_color(&r);

    //         canvas.pixels[j * IMAGE_WIDTH + i] = pixel_color;
    //     }
    // }
    let mut canvas: Canvas = Canvas::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (j as f64) / ((IMAGE_WIDTH - 1) as f64);
            let b = 0.25;

            let pixel_color = color::color(
                (r * 255.999) as u8,
                (g * 255.999) as u8,
                (b * 255.999) as u8,
            );
            canvas.pixels[j * IMAGE_WIDTH + i] = pixel_color;
        }
    }

    match output::save_ppm_image(
        canvas.pixels,
        canvas.width,
        canvas.height,
        "./ray-tracer.ppm",
    ) {
        Ok(it) => it,
        Err(err) => panic!("{}", err),
    };
}
