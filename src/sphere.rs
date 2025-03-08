use crate::aabb::Aabb;
use crate::common::PI;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Material,
    bbox: Aabb,
}

impl Sphere {
    pub fn stationary(centre: Point3, radius: f64, material: Material) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            centre: Ray::new(centre, Point3::zero()),
            radius,
            material,
            bbox: Aabb::fit(centre - rvec, centre + rvec),
        }
    }

    pub fn moving(centres: (Point3, Point3), radius: f64, material: Material) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::fit(centres.0 - rvec, centres.0 + rvec);
        let box2 = Aabb::fit(centres.1 - rvec, centres.1 + rvec);
        Sphere {
            centre: Ray::new(centres.0, centres.1 - centres.0),
            radius,
            material,
            bbox: Aabb::combine(&box1, &box2),
        }
    }

    /// p: a given point on the sphere of radius one, centred at the origin.
    /// u: returned value [0,1] of angle around the Y axis from X=-1
    /// v: returned value [0,1] of angle from Y=-1 to Y=+1
    ///
    /// # Examples
    /// ```
    /// use ray_tracing::sphere::Sphere;
    /// use ray_tracing::vec3::Point3;
    ///
    /// let (u, v) = Sphere::get_uv(&Point3::new(1.0, 0.0, 0.0));
    /// assert_eq!((u, v), (0.5, 0.5));
    ///
    /// let (u, v) = Sphere::get_uv(&Point3::new(-1.0, 0.0, 0.0));
    /// assert_eq!((u, v), (0.0, 0.5));
    ///
    /// let (u, v) = Sphere::get_uv(&Point3::new(0.0, 1.0, 0.0));
    /// assert_eq!((u, v), (0.5, 1.0));
    ///
    /// let (u, v) = Sphere::get_uv(&Point3::new(0.0, -1.0, 0.0));
    /// assert_eq!((u, v), (0.5, 0.0));
    ///
    /// let (u, v) = Sphere::get_uv(&Point3::new(0.0, 0.0, 1.0));
    /// assert_eq!((u, v), (0.25, 0.5));
    ///
    /// let (u, v) = Sphere::get_uv(&Point3::new(0.0, 0.0, -1.0));
    /// assert_eq!((u, v), (0.75, 0.5));
    /// ```
    pub fn get_uv(p: &Point3) -> (f64, f64) {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        (phi / (2.0 * PI), (theta / PI))
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
        (rec.u, rec.v) = Sphere::get_uv(&outward_normal);
        rec.set_face_normal(r, outward_normal);

        Some((rec, &self.material))
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
