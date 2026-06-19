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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colour::Colour;
    use crate::material::Material;

    const EPS: f64 = 1e-9;

    fn vec_approx_eq(a: Vec3, b: Vec3) {
        assert!(
            (a.x() - b.x()).abs() < EPS
                && (a.y() - b.y()).abs() < EPS
                && (a.z() - b.z()).abs() < EPS,
            "expected {} ~= {}",
            a,
            b
        );
    }

    /// A stand-in hittable with a fixed bounding box and a fixed hit record,
    /// so the rotation transforms can be checked in isolation.
    struct Dummy {
        bbox: Aabb,
        p: Point3,
        normal: Vec3,
        material: Material,
    }

    impl Dummy {
        fn new(bbox: Aabb, p: Point3, normal: Vec3) -> Self {
            Dummy {
                bbox,
                p,
                normal,
                material: Material::Metal {
                    albedo: Colour::zero(),
                    fuzziness: 0.0,
                },
            }
        }
    }

    impl Hittable for Dummy {
        fn hit(&self, _r: &Ray, _ray_t: Interval) -> Option<HitRecord<'_>> {
            let mut rec = HitRecord::new(&self.material);
            rec.t = 1.0;
            rec.p = self.p;
            rec.normal = self.normal;
            Some(rec)
        }

        fn bounding_box(&self) -> Option<Aabb> {
            Some(self.bbox)
        }
    }

    // --- the `rotate` helper (the core of the transform) ---

    #[test]
    fn rotate_y_90_degrees() {
        // +x swings to -z, +z swings to +x.
        vec_approx_eq(
            rotate(Axis::Y, 0.0, 1.0, Vec3::new(1.0, 0.0, 0.0)),
            Vec3::new(0.0, 0.0, -1.0),
        );
        vec_approx_eq(
            rotate(Axis::Y, 0.0, 1.0, Vec3::new(0.0, 0.0, 1.0)),
            Vec3::new(1.0, 0.0, 0.0),
        );
    }

    #[test]
    fn rotate_x_90_degrees() {
        vec_approx_eq(
            rotate(Axis::X, 0.0, 1.0, Vec3::new(0.0, 1.0, 0.0)),
            Vec3::new(0.0, 0.0, 1.0),
        );
    }

    #[test]
    fn rotate_z_90_degrees() {
        vec_approx_eq(
            rotate(Axis::Z, 0.0, 1.0, Vec3::new(1.0, 0.0, 0.0)),
            Vec3::new(0.0, 1.0, 0.0),
        );
    }

    #[test]
    fn rotate_then_inverse_round_trips() {
        // cos/sin of some valid rotation (0.8^2 + 0.6^2 == 1).
        let (c, s) = (0.8, 0.6);
        let v = Vec3::new(0.3, -0.7, 0.5);
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let forward = rotate(axis, c, s, v);
            let back = rotate(axis, c, -s, forward); // inverse negates sin
            vec_approx_eq(back, v);
        }
    }

    // --- the `Rotate` hittable wrapper (bbox + hit-record transform) ---

    #[test]
    fn bounding_box_of_y_rotated_unit_box() {
        // Unit box [0,1]^3 rotated 90 deg about Y maps (x,y,z) -> (z, y, -x),
        // so the new box is x in [0,1], y in [0,1], z in [-1,0].
        let inner = Dummy::new(
            Aabb::new(Point3::zero(), Point3::new(1.0, 1.0, 1.0)),
            Point3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let rotated = Rotate::new(inner, Axis::Y, 90.0);
        let bbox = rotated.bounding_box().unwrap();
        vec_approx_eq(bbox.min(), Point3::new(0.0, 0.0, -1.0));
        vec_approx_eq(bbox.max(), Point3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn hit_rotates_point_and_normal_back_to_world_space() {
        // Object-space hit at p=(1,0,0) with normal=(1,0,0); after a 90 deg Y
        // rotation both should map to (0,0,-1) in world space.
        let inner = Dummy::new(
            Aabb::new(Point3::zero(), Point3::new(1.0, 1.0, 1.0)),
            Point3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let rotated = Rotate::new(inner, Axis::Y, 90.0);

        let r = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let rec = rotated
            .hit(&r, Interval::new(0.001, common::INFINITY))
            .expect("dummy always reports a hit");

        vec_approx_eq(rec.p, Vec3::new(0.0, 0.0, -1.0));
        vec_approx_eq(rec.normal, Vec3::new(0.0, 0.0, -1.0));
    }
}
