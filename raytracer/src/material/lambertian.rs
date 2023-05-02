use crate::{hittable::HitRecord, material::Material, ray::Ray};
use vec3::Color;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // If the random unit vector we generate is exactly opposite the normal vector,
        // the two will sum to zero, which will result in a zero scatter direction vector.
        // This leads to bad scenarios later on (infinities and NaNs),
        // so we need to intercept the condition before we pass it on.
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = (rec.p, scatter_direction).into();
        *attenuation = self.albedo;
        true
    }
}

impl Lambertian {
    pub fn new_with(albedo: Color) -> Self {
        Self { albedo }
    }
}
