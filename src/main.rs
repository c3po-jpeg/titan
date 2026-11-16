use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};

use math::vec3::Vec3;

mod hitlist;
mod hittable;
mod ray;
mod sphere;

use crate::{hitlist::HitList, hittable::Hittable, ray::Ray, sphere::Sphere};

const WIDTH: u32 = 800;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
const TOTAL: u32 = WIDTH * HEIGHT;

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (WIDTH as f32 / HEIGHT as f32);

const FOCAL_LENGTH: f32 = 1.0;

// background color based on ray direction
fn ray_color(ray: &Ray, world: &HitList) -> Rgb<u8> {
    if let Some(rec) = world.hit(ray, 0.0, f32::INFINITY) {
        return Rgb([
            (rec.normal.x * 255.0) as u8,
            (rec.normal.y * 255.0) as u8,
            (rec.normal.z * 255.0) as u8,
        ]);
    }

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

fn main() {
    let eye = Vec3::new(0.0, 0.0, 1.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let top_left = eye - (horizontal * 0.5) + (vertical * 0.5) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

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

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f32 / (WIDTH - 1) as f32;
        let v = y as f32 / (HEIGHT - 1) as f32;
        let ray = Ray::new(eye, top_left + (horizontal * u) - (vertical * v) - eye);
        *pixel = ray_color(&ray, &world);

        progress.inc(1);
    }

    progress.finish_with_message("done");

    img.save("output.png").expect("failed to save image");
}
