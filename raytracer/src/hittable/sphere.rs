use std::sync::Arc;

use vec3::{dot, Vec3};

use crate::material::Material;

use super::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Box<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
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

        let out_normal = (r.at(root) - self.center) / self.radius;
        let mut rec: HitRecord = HitRecord {
            t: root,
            p: r.at(root),
            normal: Default::default(),
            front_face: false,
            material: self.material.clone(),
        };
        rec.set_face_normal(r, out_normal);

        Some(rec)
    }
}

impl Sphere {
    pub fn new_with(center: Vec3, radius: f64, material: Arc<Box<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            material: material.clone(),
        }
    }
}

impl From<((f64, f64, f64), f64, Arc<Box<dyn Material>>)> for Sphere {
    fn from((center, radius, material): ((f64, f64, f64), f64, Arc<Box<dyn Material>>)) -> Self {
        Self::new_with(center.into(), radius, material)
    }
}
