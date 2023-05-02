use crate::{hittable::HitRecord, material::Material, ray::Ray};
use vec3::Color;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(&vec3::unit_vector(&r_in.direction()), &rec.normal);
        *scattered = (rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere()).into();
        *attenuation = self.albedo;
        vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new_with(albedo: Color, f: f64) -> Self {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Self { albedo, fuzz }
    }
}
