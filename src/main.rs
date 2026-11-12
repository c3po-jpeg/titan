use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};

use math::vec3::Vec3;

mod ray;

use crate::ray::Ray;

const WIDTH: u32 = 800;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
const TOTAL: u32 = WIDTH * HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

// background color based on ray direction
fn ray_color(ray: &Ray) -> Rgb<u8> {
    let dir = ray.direction.normalize();
    let b = 0.5 * (dir.y + 1.0);
    let white = Vec3::ONE;
    let blue = Vec3::new(0.5, 0.7, 1.0);
    let background = (white * (1.0 - b)) + (blue * b);

    Rgb([
        (background.x * 255.0) as u8,
        (background.y * 255.0) as u8,
        (background.z * 255.0) as u8,
    ])
}

fn hit_sphere(ray: &Ray, center: Vec3, radius: f32) -> bool {
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * (Vec3::dot(&ray.origin, &ray.direction) - Vec3::dot(&ray.direction, &center));
    let c = Vec3::dot(&ray.origin, &ray.origin) - 2.0 * Vec3::dot(&ray.origin, &center)
        + Vec3::dot(&center, &center)
        - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        return true;
    }
    false
}

fn main() {
    let eye = Vec3::new(0.0, 0.0, 1.0);
    let horizontal = Vec3::new(WIDTH as f32, 0.0, 0.0);
    let vertical = Vec3::new(0.0, HEIGHT as f32, 0.0);
    let top_left = eye - (horizontal * 0.5) + (vertical * 0.5) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    let progress = ProgressBar::new(TOTAL as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=> "),
    );

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f32 / (WIDTH - 1) as f32;
        let v = y as f32 / (HEIGHT - 1) as f32;
        let ray = Ray::new(eye, top_left + (horizontal * u) - (vertical * v) - eye);
        *pixel = ray_color(&ray);

        progress.inc(1);
    }

    progress.finish_with_message("done");

    img.save("output.png").expect("failed to save image");
}
