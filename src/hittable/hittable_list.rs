use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn from(objects: Vec<Box<dyn Hittable + Sync + Send>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: impl Hittable + Sync + Send + 'static) {
        self.objects.push(Box::new(object))
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
}
