use crate::common;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::ops::Add;

#[derive(Default, Clone)]
pub struct Aabb {
    min: Point3,
    max: Point3,
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Aabb {
        Aabb {
            min: minimum,
            max: maximum,
        }
        .pad_to_minimums()
    }

    pub fn fit(a: Point3, b: Point3) -> Aabb {
        let small = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let big = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        Aabb::new(small, big).pad_to_minimums()
    }

    pub fn empty() -> Aabb {
        Aabb {
            min: Point3::new(common::INFINITY, common::INFINITY, common::INFINITY),
            max: Point3::new(-common::INFINITY, -common::INFINITY, -common::INFINITY),
        }
    }

    fn pad_to_minimums(&self) -> Self {
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
        let small = Point3::new(
            a.min().x().min(b.min().x()),
            a.min().y().min(b.min().y()),
            a.min().z().min(b.min().z()),
        );

        let big = Point3::new(
            a.max().x().max(b.max().x()),
            a.max().y().max(b.max().y()),
            a.max().z().max(b.max().z()),
        );

        Aabb::new(small, big)
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

            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                (t1, t0) = (t0, t1); // Swap t0 and t1
            }

            let t_min_temp = if t0 > ray_t.min() { t0 } else { ray_t.min() };
            let t_max_temp = if t1 < ray_t.max() { t1 } else { ray_t.max() };

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
