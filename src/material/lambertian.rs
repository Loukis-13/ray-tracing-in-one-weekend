use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{self, Color};

use super::material::Material;

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn from(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray { origin: rec.p, direction: scatter_direction };
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
