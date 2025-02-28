use crate::common::INFINITY;

#[derive(PartialEq, Copy, Clone)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn combine(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: if a.min() <= b.min() {a.min()} else {b.min()},
            max: if a.max() >= b.max() {a.max()} else {b.max()},
        }
    }

    pub fn ordered(v1: f64, v2: f64) -> Self {
        if v1 <= v2 {
            Interval { min: v1, max: v2 }
        } else {
            Interval { min: v2, max: v1 }
        }
    }

    pub fn empty() -> Self {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn universe() -> Self {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }
}
