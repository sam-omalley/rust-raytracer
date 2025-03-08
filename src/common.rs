use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use std::cell::RefCell;

// Constants

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

// Utility functions

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_os_rng());
}

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_float() -> f32 {
    RNG.with(|rng| rng.borrow_mut().random())
}

#[inline]
pub fn random_int(min: i32, max: i32) -> i32 {
    RNG.with(|rng| rng.borrow_mut().random_range(min..=max))
}

#[inline]
pub fn random_float_range(min: f32, max: f32) -> f32 {
    // Return a random real in [min, max)
    min + (max - min) * random_float()
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
