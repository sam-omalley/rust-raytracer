use crate::common::INFINITY;

#[derive(PartialEq, Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn combine(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: if a.min() <= b.min() { a.min() } else { b.min() },
            max: if a.max() >= b.max() { a.max() } else { b.max() },
        }
    }

    pub fn ordered(v1: f32, v2: f32) -> Self {
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

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn clamp(&self, v: f32) -> f32 {
        if v < self.min() {
            self.min()
        } else if v > self.max() {
            self.max()
        } else {
            v
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
