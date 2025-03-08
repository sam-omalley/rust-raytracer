use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Default, Debug)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn push(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
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
                res = Some((hit_record, material));
            }
        }
        res
    }

    // TODO: Update bounding_box() to return Option<Aabb>
    fn bounding_box(&self) -> Aabb {
        match self.objects.first() {
            Some(first) => {
                let bbox = first.bounding_box();
                self.objects.iter().skip(1).fold(bbox, |acc, hittable| {
                    let bbox = hittable.bounding_box();
                    Aabb::combine(&acc, &bbox)
                })
            }
            _ => Aabb::empty(),
        }
    }
}
