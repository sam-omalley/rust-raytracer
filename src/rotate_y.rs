use crate::aabb::Aabb;
use crate::common;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// TODO: Add Axis rather than hard-code Y-axis.
#[derive(Debug)]
pub struct RotateY<H: Hittable> {
    object: H,
    bbox: Aabb,
    sin_theta: f32,
    cos_theta: f32,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(object: H, angle: f32) -> Self {
        let radians = common::degrees_to_radians(angle);
        let sin_theta = f32::sin(radians);
        let cos_theta = f32::cos(radians);
        let bbox = object.bounding_box();

        let mut min = Point3::fill(common::INFINITY);
        let mut max = Point3::fill(-common::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.max().x() + (1 - i) as f32 * bbox.min().x();
                    let y = j as f32 * bbox.max().y() + (1 - j) as f32 * bbox.min().y();
                    let z = k as f32 * bbox.max().z() + (1 - k) as f32 * bbox.min().z();

                    let x = cos_theta * x + sin_theta * z;
                    let z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(x, y, z);

                    for c in 0..3 {
                        min[c] = f32::min(min[c], tester[c]);
                        max[c] = f32::max(min[c], tester[c]);
                    }
                }
            }
        }

        let bbox = Aabb::new(min, max);

        Self {
            object,
            bbox,
            sin_theta,
            cos_theta,
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'a>> {
        fn rot(p: Vec3, sin_theta: f32, cos_theta: f32) -> Vec3 {
            Vec3::new(
                p.dot(Vec3::new(cos_theta, 0.0, sin_theta)),
                p.dot(Vec3::new(0.0, 1.0, 0.0)),
                p.dot(Vec3::new(-sin_theta, 0.0, cos_theta)),
            )
        }

        let rot_ray = Ray {
            orig: rot(r.orig, -self.sin_theta, self.cos_theta),
            dir: rot(r.dir, -self.sin_theta, self.cos_theta),
            ..*r
        };

        self.object.hit(&rot_ray, ray_t).map(|hit| HitRecord {
            p: rot(hit.p, self.sin_theta, self.cos_theta),
            normal: rot(hit.normal, self.sin_theta, self.cos_theta),
            ..hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

#[inline]
pub fn rotate_y<H: Hittable>(degrees: f32, object: H) -> RotateY<H> {
    RotateY::new(object, degrees)
}
