use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::random_unit_vector;
use crate::{common, vec3};

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian { texture: Texture },
    Metal { albedo: Colour, fuzziness: f64 },
    Dielectric { refraction: f64 },
    DiffuseLight { texture: Texture },
    Isotropic { texture: Texture },
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
            Material::Lambertian { texture } => {
                let mut scatter_direction = rec.normal + vec3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                Some((
                    texture.colour(rec.u, rec.v, rec.p),
                    Ray::new_at(rec.p, scatter_direction, r_in.time()),
                ))
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
            Material::Dielectric { refraction } => {
                let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(rec.normal) > 0.0
                {
                    (
                        -rec.normal,
                        *refraction,
                        *refraction * r_in.direction().dot(rec.normal) / r_in.direction().length(),
                    )
                } else {
                    (
                        rec.normal,
                        1.0 / *refraction,
                        -r_in.direction().dot(rec.normal) / r_in.direction().length(),
                    )
                };

                let direction = vec3::refract(r_in.direction(), outward_normal, ni_over_nt)
                    .filter(|_| common::random_double() >= reflectance(cosine, *refraction))
                    .unwrap_or_else(|| vec3::reflect(r_in.direction(), rec.normal));

                let attenuation = Colour::fill(1.0);
                let ray = Ray::new_at(rec.p, direction, r_in.time());
                Some((attenuation, ray))
            }
            Material::DiffuseLight { texture: _ } => None,
            Material::Isotropic { texture } => {
                let scattered = Ray::new_at(rec.p, random_unit_vector(), r_in.time());
                let attenuation = texture.colour(rec.u, rec.v, rec.p);
                Some((attenuation, scattered))
            }
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: vec3::Point3) -> Colour {
        match self {
            Material::DiffuseLight { texture } => texture.colour(u, v, p),
            _ => Colour::zero(),
        }
    }
}
