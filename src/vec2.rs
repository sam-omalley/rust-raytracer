use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(PartialEq, Copy, Clone, Default)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y)
    }
}

// Type alias
pub type Point2 = Vec2;

// Output formatting
impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

// ~Vec2
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

// Vec2 += Vec2
impl AddAssign for Vec2 {
    fn add_assign(&mut self, v: Vec2) {
        *self = *self + v;
    }
}

// Vec2 *= f64
impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

// Vec2 /= f64
impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

// Vec2 + Vec2
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, v: Vec2) -> Vec2 {
        Vec2::new(self.x() + v.x(), self.y() + v.y())
    }
}

// Vec2 - Vec2
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, v: Vec2) -> Vec2 {
        Vec2::new(self.x() - v.x(), self.y() - v.y())
    }
}

// Vec2 * Vec2
impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        Vec2::new(self.x() * v.x(), self.y() * v.y())
    }
}

// f64 * Vec2
impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        v * self
    }
}

// Vec2 * f64
impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, t: f64) -> Vec2 {
        Vec2::new(self.x() * t, self.y() * t)
    }
}

// Vec2 / f64
impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, t: f64) -> Vec2 {
        Vec2::new(self.x() / t, self.y() / t)
    }
}

pub fn dot(u: Vec2, v: Vec2) -> f64 {
    (u.x * v.x) + (u.y * v.y)
}

pub fn unit_vector(v: Vec2) -> Vec2 {
    v / v.length()
}
