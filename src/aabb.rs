use crate::common;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::ops::Add;

#[derive(Default, Clone, Copy)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Aabb {
        Aabb {
            min: minimum,
            max: maximum,
        }
    }

    pub fn fit(a: Point3, b: Point3) -> Aabb {
        let small = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let big = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        Aabb::new(small, big)
    }

    pub fn empty() -> Aabb {
        Aabb {
            min: Point3::new(common::INFINITY, common::INFINITY, common::INFINITY),
            max: Point3::new(-common::INFINITY, -common::INFINITY, -common::INFINITY),
        }
    }

    pub fn pad_to_minimums(&self) -> Self {
        let delta: f64 = 0.0001;

        let mut delta_x = 0.0;
        let mut delta_y = 0.0;
        let mut delta_z = 0.0;

        if self.max.x() - self.min.x() < delta {
            delta_x = delta / 2.0;
        }
        if self.max.y() - self.min.y() < delta {
            delta_y = delta / 2.0;
        }
        if self.max.z() - self.min.z() < delta {
            delta_z = delta / 2.0;
        }
        let delta = Point3::new(delta_x, delta_y, delta_z);

        Aabb {
            min: self.min - delta,
            max: self.max + delta,
        }
    }

    pub fn combine(a: &Aabb, b: &Aabb) -> Aabb {
        let min = Point3::new(
            f64::min(a.min().x(), b.min().x()),
            f64::min(a.min().y(), b.min().y()),
            f64::min(a.min().z(), b.min().z()),
        );

        let max = Point3::new(
            f64::max(a.max().x(), b.max().x()),
            f64::max(a.max().y(), b.max().y()),
            f64::max(a.max().z(), b.max().z()),
        );

        Aabb::new(min, max)
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn longest_axis(&self) -> usize {
        let mut longest_axis = 0;
        let mut longest = 0.0;
        for axis in 0..=2 {
            let sz = f64::abs(self.max[axis] - self.min[axis]);
            if sz > longest {
                longest = sz;
                longest_axis = axis;
            }
        }
        longest_axis
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        for a in 0..=2 {
            let inv_d = 1.0 / r.direction()[a];

            let t0 = (self.min[a] - r.origin()[a]) * inv_d;
            let t1 = (self.max[a] - r.origin()[a]) * inv_d;

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            let t_min_temp = ray_t.min().max(t0);
            let t_max_temp = ray_t.max().min(t1);

            if t_max_temp <= t_min_temp {
                return false;
            }
        }

        true
    }
}

impl Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, offset: Vec3) -> Aabb {
        let min = self.min() + offset;
        let max = self.max() + offset;
        Aabb::new(min, max)
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, bbox: Aabb) -> Aabb {
        bbox + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;

    fn unit_box() -> Aabb {
        Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0))
    }

    fn full() -> Interval {
        Interval::new(0.001, common::INFINITY)
    }

    #[test]
    fn ray_pointing_at_box_hits() {
        let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(unit_box().hit(&r, full()));
    }

    #[test]
    fn ray_pointing_away_misses() {
        let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(!unit_box().hit(&r, full()));
    }

    #[test]
    fn ray_parallel_and_offset_misses() {
        // Travels along +z but is offset in x so it never enters the slab.
        let r = Ray::new(Point3::new(5.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(!unit_box().hit(&r, full()));
    }

    #[test]
    fn ray_from_inside_hits() {
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(unit_box().hit(&r, full()));
    }

    #[test]
    fn diagonal_ray_through_corner_hits() {
        let r = Ray::new(Point3::new(-5.0, -5.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(unit_box().hit(&r, full()));
    }

    #[test]
    fn hit_respects_ray_interval() {
        // Box is in front, but the interval ends before the ray reaches it.
        let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(!unit_box().hit(&r, Interval::new(0.001, 1.0)));
    }
}
