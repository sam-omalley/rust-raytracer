use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn stationary(centre: Point3, radius: f64, material: Material) -> Sphere {
        Sphere {
            centre: Ray::new(centre, Point3::zero()),
            radius,
            material,
        }
    }

    pub fn moving(centres: (Point3, Point3), radius: f64, material: Material) -> Sphere {
        Sphere {
            centre: Ray::new(centres.0, centres.1 - centres.0),
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, &Material)> {
        let current_centre = self.centre.at(r.time());
        let oc = r.origin() - current_centre;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = f64::sqrt(discriminant);

        // Find the nearest root that lies in the aceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_centre) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some((rec, &self.material))
    }
}
