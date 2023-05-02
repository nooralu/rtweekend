use vec3::{Point3, Vec3};

use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin: Point3 = (0.0, 0.0, 0.0).into();
        let horizontal: Vec3 = (viewport_width, 0.0, 0.0).into();
        let vertical: Vec3 = (0.0, viewport_height, 0.0).into();
        let lower_left_corner: Point3 =
            origin - horizontal / 2.0 - vertical / 2.0 - (0.0, 0.0, focal_length).into();

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn new_with(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        // vertical field-of-view in degrees
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(&(lookfrom - lookat));
        let u = vec3::unit_vector(&vec3::cross(&vup, &w));
        let v = vec3::cross(&w, &u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        (
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
            .into()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
