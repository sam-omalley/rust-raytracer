use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

use std::sync::Arc;

#[derive(Default, Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = Aabb::combine(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }

    pub fn objects(&self) -> Vec<Arc<dyn Hittable>> {
        self.objects.clone()
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
