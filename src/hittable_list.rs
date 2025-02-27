use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, &Material)> {
        let mut closest_so_far = ray_t.max();
        let mut res = None;

        for h in self.objects.iter() {
            if let Some((hit_record, material)) =
                h.hit(r, Interval::new(ray_t.min(), closest_so_far))
            {
                closest_so_far = hit_record.t;
                res = Some((hit_record.clone(), material));
            }
        }
        res
    }
}
