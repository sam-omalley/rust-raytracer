use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// TODO: Move Material into HitRecord
#[derive(Clone)]
pub struct HitRecord<'m> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub material: &'m Material,
}

pub trait Hittable: std::fmt::Debug + Send + Sync {
    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'a>>;
    fn bounding_box(&self) -> Aabb;
}
