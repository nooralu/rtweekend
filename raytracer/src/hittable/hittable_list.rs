use super::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(temp_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }
}

impl HittableList {
    pub fn new_with(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
