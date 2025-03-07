use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::sync::Arc;

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box().clone() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, &Material)> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new_at(r.origin() - self.offset, r.direction(), r.time());

        // Determine whether an intersection exists along the offset ray (amd of so, where)
        match self.object.hit(&offset_r, ray_t) {
            None => None,
            Some((mut rec, mat)) => {
                rec.p += self.offset;
                Some((rec, mat))
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
