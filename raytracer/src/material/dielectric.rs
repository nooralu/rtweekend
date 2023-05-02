use crate::{hittable::HitRecord, material::Material, ray::Ray};
use vec3::{refract, unit_vector, Color};

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = (1.0, 1.0, 1.0).into();
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(&r_in.direction());
        let refracted = refract(&unit_direction, &rec.normal, refraction_ratio);

        *scattered = (rec.p, refracted).into();
        true
    }
}

impl Dielectric {
    pub fn new_with(ir: f64) -> Self {
        Self { ir }
    }
}
