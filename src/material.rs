use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::{common, vec3};

pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour, fuzziness: f64 },
    Dialectric { refraction: f64 },
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + vec3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                Some((*albedo, Ray::new_at(rec.p, scatter_direction, r_in.time())))
            }
            Material::Metal { albedo, fuzziness } => {
                let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
                let scattered = Ray::new_at(
                    rec.p,
                    reflected + *fuzziness * vec3::random_in_unit_sphere(),
                    r_in.time(),
                );

                if vec3::dot(scattered.direction(), rec.normal) <= 0.0 {
                    None
                } else {
                    Some((*albedo, scattered))
                }
            }
            Material::Dialectric { refraction } => {
                let refraction_ratio = if rec.front_face {
                    1.0 / *refraction
                } else {
                    *refraction
                };

                let unit_direction = vec3::unit_vector(r_in.direction());
                let cos_theta = f64::min(vec3::dot(-unit_direction, rec.normal), 1.0);
                let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > common::random_double()
                {
                    vec3::reflect(unit_direction, rec.normal)
                } else {
                    vec3::refract(unit_direction, rec.normal, refraction_ratio)
                };

                Some((
                    Colour::new(1.0, 1.0, 1.0),
                    Ray::new_at(rec.p, direction, r_in.time()),
                ))
            }
        }
    }
}
