use vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            orig: Default::default(),
            dir: Default::default(),
        }
    }

    pub fn new_with(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new()
    }
}

impl From<(Point3, Vec3)> for Ray {
    fn from(tuple: (Point3, Vec3)) -> Self {
        Self::new_with(&tuple.0, &tuple.1)
    }
}
