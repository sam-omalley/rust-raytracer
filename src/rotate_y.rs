use crate::aabb::Aabb;
use crate::common;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};


// TODO: Add Axis rather than hard-code Y-axis.
pub struct RotateY<H: Hittable> {
    object: H,
    bbox: Aabb,
    sin_theta: f64,
    cos_theta: f64,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(object: H, angle: f64) -> Self {
        let radians = common::degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let bbox = object.bounding_box();

        let mut min = Point3::fill(common::INFINITY);
        let mut max = Point3::fill(-common::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let x = cos_theta * x + sin_theta * z;
                    let z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(x, y, z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(min[c], tester[c]);
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, &Material)> {
        // Transform the ray from world space to object space.
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new_at(origin, direction, r.time());

        // Determine whether an intersection exists in object space (and if so, where).
        if let Some((mut rec, mat)) = self.object.hit(&rotated_r, ray_t) {
            rec.p = Point3::new(
                (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
                rec.p.y(),
                (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
            );

            rec.normal = Vec3::new(
                (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
                rec.p.y(),
                (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
            );

            return Some((rec, mat));
        }
        None
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
