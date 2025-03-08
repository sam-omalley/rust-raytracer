use crate::common;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { e: [x, y, z] }
    }

    #[inline]
    pub fn newi(x: i32, y: i32, z: i32) -> Self {
        Self {
            e: [x as f32, y as f32, z as f32],
        }
    }

    #[inline]
    pub fn fill(t: f32) -> Self {
        Vec3 { e: [t, t, t] }
    }

    #[inline]
    pub fn zero() -> Self {
        Vec3::fill(0.0)
    }

    #[inline]
    pub fn random() -> Self {
        Self {
            e: [
                common::random_float(),
                common::random_float(),
                common::random_float(),
            ],
        }
    }

    #[inline]
    pub fn random_range(min: f32, max: f32) -> Self {
        Self {
            e: [
                common::random_float_range(min, max),
                common::random_float_range(min, max),
                common::random_float_range(min, max),
            ],
        }
    }

    #[inline]
    pub fn axis(&self, n: usize) -> f32 {
        self.e[n]
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2])
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        const EPS: f32 = 1.0e-8;
        // Return true if the vector is close to zero in all dimensions.
        f32::abs(self.e[0]) < EPS && f32::abs(self.e[1]) < EPS && f32::abs(self.e[2]) < EPS
    }
    #[inline]
    pub fn dot(&self, v: Vec3) -> f32 {
        (self.e[0] * v.e[0]) + (self.e[1] * v.e[1]) + (self.e[2] * v.e[2])
    }
}

// Type alias
pub type Point3 = Vec3;

// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// ~Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

// Vec3 += Vec3
impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

// Vec3 *= f32
impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

// Vec3 /= f32
impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, t: f32) {
        *self = *self / t;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

// f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// Vec3 * f32
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, t: f32) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

// Vec3 / f32
impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, t: f32) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

// f32 / Vec3
impl Div<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self / v.x(), self / v.y(), self / v.z())
    }
}

impl std::iter::Sum for Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::default(), std::ops::Add::add)
    }
}

#[inline]
pub fn dot(u: Vec3, v: Vec3) -> f32 {
    (u.e[0] * v.e[0]) + (u.e[1] * v.e[1]) + (u.e[2] * v.e[2])
}

#[inline]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[inline]
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

#[inline]
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            common::random_float_range(-1.0, 1.0),
            common::random_float_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

/// Refracts `v` into a material with surface normal `n`. `ni_over_nt` is the
/// refractive index if the ray is exiting the material, or its reciprocal if
/// it's entering.
#[inline]
pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = unit_vector(v);
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}
