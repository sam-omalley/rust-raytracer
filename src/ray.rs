use crate::vec3::{Point3, Vec3};

#[derive(Default, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            time: 0.0,
        }
    }

    #[inline]
    pub fn new_at(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            time,
        }
    }

    #[inline]
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    #[inline]
    pub fn time(&self) -> f64 {
        self.time
    }

    #[inline]
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
