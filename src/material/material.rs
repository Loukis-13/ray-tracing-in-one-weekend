use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::color::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
