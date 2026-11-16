use math::vec3::Vec3;

use crate::ray::Ray;

pub struct Camera {
    eye: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    top_left: Vec3,

    /* focal_length: f32,
    viewport_width: f32,
    viewport_height: f32,
    aspect_ratio: f32, */
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Vec3::Z, 1.0, 2.0, 16.0 / 9.0)
    }
}

impl Camera {
    pub fn new(eye: Vec3, focal_length: f32, viewport_height: f32, aspect_ratio: f32) -> Self {
        let viewport_width = viewport_height * aspect_ratio;
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let mut top_left = eye - (0.5 * horizontal);
        top_left = top_left + (0.5 * vertical);
        top_left = top_left - Vec3::new(0.0, 0.0, focal_length);

        Self {
            eye,
            vertical,
            horizontal,
            top_left,

            /* focal_length,
            viewport_height,
            viewport_width,
            aspect_ratio, */
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let mut direction = self.top_left + u * self.horizontal;
        direction = direction - v * self.vertical;
        direction = direction - self.eye;
        Ray::new(self.eye, direction)
    }
}
