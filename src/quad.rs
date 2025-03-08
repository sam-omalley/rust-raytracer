use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};

#[derive(Debug)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
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
        let w = n / dot(n, n);

        Quad {
            q,
            u,
            v,
            w,
            material,
            bbox: Self::calc_bounding_box(q, u, v),
            normal,
            d,
        }
    }

    fn calc_bounding_box(q: Point3, u: Vec3, v: Vec3) -> Aabb {
        let bbox_diagonal1 = Aabb::new(q, q + u + v).pad_to_minimums();
        let bbox_diagonal2 = Aabb::new(q + u, q + v).pad_to_minimums();
        Aabb::combine(&bbox_diagonal1, &bbox_diagonal2)
    }

    fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        let unit_interval = Interval::new(0.0, 1.0);

        // Given the hit point in plane coordinates, return None if it is
        // outside the primitive, otherwise return the UV coordinates.
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        }

        Some((a, b))
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

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));

        if let Some((u, v)) = Self::is_interior(alpha, beta) {
            let mut rec = HitRecord::new();
            rec.u = u;
            rec.v = v;
            rec.t = t;
            rec.p = intersection;
            rec.set_face_normal(r, self.normal);

            return Some((rec, &self.material));
        }
        None
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub fn quad_box(a: Point3, b: Point3, material: Material) -> HittableList {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Point3::new(
        f64::min(a.x(), b.x()),
        f64::min(a.y(), b.y()),
        f64::min(a.z(), b.z()),
    );
    let max = Point3::new(
        f64::max(a.x(), b.x()),
        f64::max(a.y(), b.y()),
        f64::max(a.z(), b.z()),
    );

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    // Front
    sides.push(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        material.clone(),
    ));
    // Right
    sides.push(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        material.clone(),
    ));
    // Back
    sides.push(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        material.clone(),
    ));
    // Left
    sides.push(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        material.clone(),
    ));
    // Top
    sides.push(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        material.clone(),
    ));
    // Bottom
    sides.push(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        material.clone(),
    ));

    sides
}
