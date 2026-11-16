use crate::hittable::{HitRecord, Hittable};

pub struct HitList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Default for HitList {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl HitList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                hit_record = Some(rec.clone());
                closest_so_far = rec.t;
            }
        }

        return hit_record;
    }
}
