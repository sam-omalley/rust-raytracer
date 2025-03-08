use crate::aabb::Aabb;
use crate::common;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, texture: Texture) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Material::Isotropic { texture },
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, &Material)> {
        let (mut rec1, _) = self.boundary.hit(r, Interval::universe())?;
        let (mut rec2, _) = self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, common::INFINITY))?;

        if rec1.t < ray_t.min() {
            rec1.t = ray_t.min();
        }
        if rec2.t > ray_t.max() {
            rec2.t = ray_t.max();
        }
        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * common::random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut rec = HitRecord::new();
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, Vec3::newi(1, 0, 0));

        Some((rec, &self.phase_function))
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
