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

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        (
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
            .into()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
