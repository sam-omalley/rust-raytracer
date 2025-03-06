use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    material: Material,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Material) -> Self {
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, q);

        Quad {
            q,
            u,
            v,
            material,
            bbox: Self::calc_bounding_box(q, u, v),
            normal,
            d,
        }
    }

    fn calc_bounding_box(q: Point3, u: Vec3, v: Vec3) -> Aabb {
        let bbox_diagonal1 = Aabb::new(q, q + u + v);
        let bbox_diagonal2 = Aabb::new(q + u, q + v);
        Aabb::combine(&bbox_diagonal1, &bbox_diagonal2)
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, &Material)> {
        let denom = dot(self.normal, r.direction());

        if f64::abs(denom) < 1e-8 {
            return None;
        }

        let t = (self.d - dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let mut rec = HitRecord::new();
        rec.t = t;
        rec.p = intersection;
        rec.set_face_normal(r, self.normal);

        Some((rec, &self.material))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
