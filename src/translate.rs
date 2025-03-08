use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Translate<H: Hittable> {
    object: H,
    offset: Vec3,
    bbox: Aabb,
}

impl<H: Hittable> Translate<H> {
    pub fn new(object: H, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
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

    fn bounding_box(&self) ->Aabb {
        self.bbox
    }
}
