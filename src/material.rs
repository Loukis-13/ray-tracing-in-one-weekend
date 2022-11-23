use rand::random;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{self, Color};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

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

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn from(index_of_refraction: f64) -> Self {
        Self { ir: index_of_refraction }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::from(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = vec3::unit_vector(r_in.direction);
        let cos_theta = vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f64>() {
            vec3::reflect(&unit_direction, &rec.normal)
        } else {
            vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::from(rec.p, direction);
        Some((scattered, attenuation))
    }
}
