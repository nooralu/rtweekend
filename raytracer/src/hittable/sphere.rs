use vec3::{dot, Vec3};

use super::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let out_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, out_normal);

        true
    }
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Default::default(),
            radius: 0.0,
        }
    }

    pub fn new_with(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl From<((f64, f64, f64), f64)> for Sphere {
    fn from(((x, y, z), radius): ((f64, f64, f64), f64)) -> Self {
        Self::new_with((x, y, z).into(), radius)
    }
}
