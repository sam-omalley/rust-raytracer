use crate::vec3::{Point3, Vec3};

#[derive(Default, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f32,
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
    pub fn new_at(origin: Point3, direction: Vec3, time: f32) -> Ray {
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
    pub fn time(&self) -> f32 {
        self.time
    }

    #[inline]
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
