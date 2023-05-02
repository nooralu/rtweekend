use vec3::Color;

use crate::{hittable::HitRecord, ray::Ray};

pub mod lambertian;
pub mod metal;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
