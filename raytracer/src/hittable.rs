use std::sync::Arc;

use vec3::{dot, Vec3};

use crate::material::Material;
use crate::ray::Ray;

pub mod hittable_list;
pub mod sphere;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    // p is the point where the ray hits the sphere
    pub p: Vec3,
    // normal is the normal vector of the sphere at the point p
    pub normal: Vec3,
    // t is the distance from the ray origin to the point p
    pub t: f64,
    // front_face is true if the ray is outside the sphere
    pub front_face: bool,
    pub material: Arc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // If the ray is inside the sphere, the normal points outward
        // from the center of the sphere, so the dot product is positive.
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
