use crate::aabb::Aabb;
use crate::common;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// X and Z are part of the general rotation API but unused by the current
// scenes, which only rotate about Y. Kept for completeness.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Rotate `v` by the angle whose cosine/sine are given, about `axis`.
/// Pass a negated `sin_theta` to apply the inverse (world -> object) rotation.
fn rotate(axis: Axis, cos_theta: f64, sin_theta: f64, v: Vec3) -> Vec3 {
    let (c, s) = (cos_theta, sin_theta);
    match axis {
        Axis::X => Vec3::new(v.x(), c * v.y() - s * v.z(), s * v.y() + c * v.z()),
        Axis::Y => Vec3::new(c * v.x() + s * v.z(), v.y(), -s * v.x() + c * v.z()),
        Axis::Z => Vec3::new(c * v.x() - s * v.y(), s * v.x() + c * v.y(), v.z()),
    }
}

pub struct Rotate<H: Hittable> {
    object: H,
    axis: Axis,
    bbox: Aabb,
    sin_theta: f64,
    cos_theta: f64,
}

impl<H: Hittable> Rotate<H> {
    pub fn new(object: H, axis: Axis, angle: f64) -> Self {
        let radians = common::degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let bbox = object.bounding_box().unwrap_or_else(Aabb::empty);

        let mut min = Point3::fill(common::INFINITY);
        let mut max = Point3::fill(-common::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let tester = rotate(axis, cos_theta, sin_theta, Vec3::new(x, y, z));

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        let bbox = Aabb::new(min, max);

        Self {
            object,
            axis,
            bbox,
            sin_theta,
            cos_theta,
        }
    }
}

impl<H: Hittable> Hittable for Rotate<H> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        // Transform the ray from world space to object space (inverse rotation).
        let origin = rotate(self.axis, self.cos_theta, -self.sin_theta, r.origin());
        let direction = rotate(self.axis, self.cos_theta, -self.sin_theta, r.direction());

        let rotated_r = Ray::new_at(origin, direction, r.time());

        // Determine whether an intersection exists in object space (and if so, where).
        if let Some(mut rec) = self.object.hit(&rotated_r, ray_t) {
            // Transform the hit back from object space to world space (forward rotation).
            rec.p = rotate(self.axis, self.cos_theta, self.sin_theta, rec.p);
            rec.normal = rotate(self.axis, self.cos_theta, self.sin_theta, rec.normal);

            return Some(rec);
        }
        None
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}
