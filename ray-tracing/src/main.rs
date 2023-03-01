mod ray;
mod vec3;
use argel::canvas::Canvas;

use crate::ray::Ray;
use crate::vec3::Vec3;

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit(r.direction().clone());
    let t = 0.5 * (unit_direction.y + 1.0);
    let color1 = Vec3::new(1.0, 1.0, 1.0);
    let color2 = Vec3::new(0.5, 0.7, 1.0);
    return (1.0 - t) * color1 + t * color2;
}

fn main() {
    // Image
    const APECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / APECT_RATIO) as usize;

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = APECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertiacal = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertiacal / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    let canvas = Canvas::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_HEIGHT {
        }
    }

    let ray = Ray::new();
    println!("{:?}", ray.at(0.5));
}
