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
        let cos_theta = vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract {
            direction = vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = (rec.p, direction).into();
        true
    }
}

impl Dielectric {
    pub fn new_with(ir: f64) -> Self {
        Self { ir }
    }
}
