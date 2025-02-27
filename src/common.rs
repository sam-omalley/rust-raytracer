use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use std::cell::RefCell;

// Constants

pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

// Utility functions

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_os_rng());
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    RNG.with(|rng| rng.borrow_mut().random())
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Return a random real in [min, max)
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
