use math::vec3::Vec3;

use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        //rec: &mut crate::hittable::HitRecord,
    ) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Vec3::dot(&oc, &ray.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut t = (-b - f32::sqrt(discriminant)) / (2.0 * a);

        if t < t_min || t_max < t {
            t = (-b + f32::sqrt(discriminant)) / (2.0 * a);
            if t < t_min || t_max < t {
                return None;
            }
        }
        let mut rec = HitRecord::default();
        rec.t = t;
        rec.p = ray.at(t);
        let normal = ((rec.p - self.center) / self.radius).normalize();
        rec.set_face_normal(ray, &normal);
        return Some(rec);
    }
}
