use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3;
use crate::color::Color;

use super::material::Material;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn from(a: Color, f: f64) -> Self {
        Self { albedo: a, fuzz: f.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = vec3::reflect(&vec3::unit_vector(r_in.direction), &rec.normal);

        let scattered = Ray {
            origin: rec.p,
            direction: reflected + vec3::random_in_unit_sphere() * self.fuzz,
        };
        let attenuation = self.albedo;

        if vec3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
