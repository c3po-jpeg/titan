use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};

use rand::RngExt;

use math::vec3::Vec3;

mod camera;
mod hitlist;
mod hittable;
mod ray;
mod sphere;

use crate::{camera::Camera, hitlist::HitList, hittable::Hittable, ray::Ray, sphere::Sphere};

const WIDTH: u32 = 800;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const TOTAL: u32 = WIDTH * HEIGHT;

// background color based on ray direction
fn ray_color(ray: &Ray, world: &HitList) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.0, f32::INFINITY) {
        return rec.normal;
    }

    let dir = ray.direction.normalize();
    let b = 0.5 * (dir.y + 1.0);
    let white = Vec3::ONE;
    let blue = Vec3::new(0.5, 0.7, 1.0);
    let background = (white * (1.0 - b)) + (blue * b);

    background
}

fn write_color(color: &Vec3, samples_per_pixel: u32) -> Rgb<u8> {
    let scale = 1.0 / samples_per_pixel as f32;

    let r = color.x * scale;
    let g = color.y * scale;
    let b = color.z * scale;

    Rgb([
        (r.clamp(0.0, 0.999) * 255.0) as u8,
        (g.clamp(0.0, 0.999) * 255.0) as u8,
        (b.clamp(0.0, 0.999) * 255.0) as u8,
    ])
}

fn main() {
    let camera = Camera::new(Vec3::Z, 1.0, 2.0, WIDTH as f32 / HEIGHT as f32);

    // world
    let mut world = HitList::default();
    world.add(Box::new(Sphere::new(-Vec3::Z, 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    let progress = ProgressBar::new(TOTAL as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=> "),
    );

    let mut rng = rand::rng();
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut color = Vec3::ZERO;
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x as f32 + rng.random::<f32>()) / (WIDTH - 1) as f32;
            let v = (y as f32 + rng.random::<f32>()) / (HEIGHT - 1) as f32;
            color = color + ray_color(&camera.ray(u, v), &world)
        }
        *pixel = write_color(&color, SAMPLES_PER_PIXEL);
        progress.inc(1);
    }

    progress.finish_with_message("done");

    img.save("output.png").expect("failed to save image");
}
