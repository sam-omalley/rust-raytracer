use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Default, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }

    pub fn fit(a: Point3, b: Point3) -> Aabb {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a particular minimum/maximum coordinate order.
        Aabb {
            x: Interval::ordered(a.x(), b.x()),
            y: Interval::ordered(a.y(), b.y()),
            z: Interval::ordered(a.z(), b.z()),
        }
    }

    pub fn combine(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb {
            x: Interval::combine(a.x(), b.x()),
            y: Interval::combine(a.y(), b.y()),
            z: Interval::combine(a.z(), b.z()),
        }
    }

    pub fn x(&self) -> &Interval {
        &self.x
    }

    pub fn y(&self) -> &Interval {
        &self.y
    }

    pub fn z(&self) -> &Interval {
        &self.z
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        let t_min: f64 = ray_t.min();
        let t_max: f64 = ray_t.max();

        // TODO: Move code into lib
        // TODO: Add unit test for hit function.

        // TODO: Replace with enum
        for axis in 0..3 {
            let &ax = self.axis_interval(axis);
            let adinv: f64 = 1.0 / ray_dir.axis(axis);

            let mut t0 = (ax.min() - ray_orig.axis(axis)) * adinv;
            let mut t1 = (ax.max() - ray_orig.axis(axis)) * adinv;

            if adinv < 0.0
            {
                (t1, t0) = (t0, t1); // Swap t0 and t1
            }

            let t_min_temp = if t0 > t_min { t0 } else { t_min };
            let t_max_temp = if t1 < t_max { t1 } else { t_max };

            if t_max_temp <= t_min_temp {
                return false;
            }
        }

        true
    }
}
