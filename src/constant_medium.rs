use crate::aabb::Aabb;
use crate::common;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct ConstantMedium<H: Hittable> {
    boundary: H,
    neg_inv_density: f64,
    phase_function: Material,
}

impl<H: Hittable> ConstantMedium<H> {
    pub fn new(boundary: H, density: f64, texture: Texture) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Material::Isotropic { texture },
        }
    }
}

impl<H: Hittable> Hittable for ConstantMedium<H> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'a>> {
        let mut rec1 = self.boundary.hit(r, Interval::universe())?;
        let mut rec2 = self
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

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);
        let normal = Vec3::newi(1, 0, 0);

        Some(HitRecord {
            t,
            p,
            normal,
            u: 0.0,
            v: 0.0,
            material: &self.phase_function,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
