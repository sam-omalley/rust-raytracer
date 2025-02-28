use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Default)]
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

    fn x(&self) -> &Interval {
        &self.x
    }

    fn y(&self) -> &Interval {
        &self.y
    }

    fn z(&self) -> &Interval {
        &self.z
    }

    fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
    fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        let mut min = ray_t.min();
        let mut max = ray_t.max();

        // TODO: Move code into lib
        // TODO: Add unit test for hit function.

        // TODO: Replace with enum
        for axis in 0..2 {
            let &ax = self.axis_interval(axis);
            let adinv: f64 = 1.0 / ray_dir.axis(axis).unwrap();

            let t0 = (ax.min() - ray_orig.axis(axis).unwrap()) * adinv;
            let t1 = (ax.max() - ray_orig.axis(axis).unwrap()) * adinv;

            if t0 < t1 {
                if t0 > min {
                    min = t0
                }
                if t1 < max {
                    max = t1
                }
            } else {
                if t1 > min {
                    min = t1
                }
                if t0 < max {
                    max = t0
                }
            }

            if max <= min {
                return false;
            }
        }

        true
    }
}
