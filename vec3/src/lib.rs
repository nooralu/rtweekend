#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

// Type aliases for vec3
pub type Point3 = Vec3; // 3D point
pub type Color = Vec3; // RGB color

pub mod ops;
pub mod utils;

pub use ops::*;
pub use utils::*;

impl Vec3 {
    pub fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn new_with(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Self(tuple.0, tuple.1, tuple.2)
    }
}
