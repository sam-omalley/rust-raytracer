use crate::common::INFINITY;

#[derive(PartialEq, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn combine(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: if a.min() <= b.min() { a.min() } else { b.min() },
            max: if a.max() >= b.max() { a.max() } else { b.max() },
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

    pub fn clamp(&self, v: f64) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_is_inclusive() {
        let i = Interval::new(1.0, 3.0);
        assert!(i.contains(1.0));
        assert!(i.contains(3.0));
        assert!(i.contains(2.0));
        assert!(!i.contains(0.999));
        assert!(!i.contains(3.001));
    }

    #[test]
    fn surrounds_is_exclusive() {
        let i = Interval::new(1.0, 3.0);
        assert!(!i.surrounds(1.0));
        assert!(!i.surrounds(3.0));
        assert!(i.surrounds(2.0));
    }

    #[test]
    fn size_is_max_minus_min() {
        assert_eq!(Interval::new(1.0, 3.0).size(), 2.0);
        assert!(Interval::empty().size() < 0.0);
    }

    #[test]
    fn ordered_sorts_endpoints() {
        let i = Interval::ordered(3.0, 1.0);
        assert_eq!((i.min(), i.max()), (1.0, 3.0));
        let j = Interval::ordered(1.0, 3.0);
        assert_eq!((j.min(), j.max()), (1.0, 3.0));
    }

    #[test]
    fn combine_spans_both() {
        let c = Interval::combine(&Interval::new(0.0, 2.0), &Interval::new(1.0, 5.0));
        assert_eq!((c.min(), c.max()), (0.0, 5.0));
        // Disjoint intervals: the union covers the outer bounds.
        let d = Interval::combine(&Interval::new(-3.0, -1.0), &Interval::new(4.0, 6.0));
        assert_eq!((d.min(), d.max()), (-3.0, 6.0));
    }

    #[test]
    fn expand_pads_both_ends_by_half_delta() {
        let e = Interval::new(1.0, 3.0).expand(2.0);
        assert_eq!((e.min(), e.max()), (0.0, 4.0));
    }

    #[test]
    fn clamp_limits_to_bounds() {
        let i = Interval::new(1.0, 3.0);
        assert_eq!(i.clamp(5.0), 3.0);
        assert_eq!(i.clamp(0.0), 1.0);
        assert_eq!(i.clamp(2.0), 2.0);
    }
}
