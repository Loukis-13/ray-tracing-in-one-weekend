use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Self { center, radius, mat_ptr }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = vec3::dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = vec3::dot(&r.direction, &outward_normal) < 0.0;

        Some(HitRecord {
            t: root,
            p,
            mat_ptr: Rc::clone(&self.mat_ptr),
            front_face,
            normal: if front_face { outward_normal } else { -outward_normal },
        })
    }
}
