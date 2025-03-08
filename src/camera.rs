use crate::colour::{self, Colour};
use crate::common;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

use rayon::prelude::*;
use std::sync::Mutex;

pub struct Render {
    pub width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    aspect_ratio: f32,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    background: Colour,
}

#[allow(clippy::too_many_arguments)]
impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32, // Vertical field-of-view in degrees
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
        background: Colour,
    ) -> Camera {
        let theta = common::degrees_to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            aspect_ratio,
            u,
            v,
            lens_radius,
            background,
        }
    }

    pub fn render(&self, world: &dyn Hittable, render: &Render) {
        let progress = Mutex::new(0);

        let height = (render.width as f32 / self.aspect_ratio) as i32;
        let num_pixels = render.width * height;

        let pixels: Vec<(u8, u8, u8)> = (0..num_pixels)
            .into_par_iter()
            .map(|index| {
                {
                    let mut count = progress.lock().unwrap();
                    *count += 1;
                    if (num_pixels - *count) % 1000 == 0 {
                        eprint!("\rScanlines remaining: {}", (num_pixels - *count));
                    }
                }

                let i = index % render.width;
                let j = height - (index / render.width) - 1;

                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..render.samples_per_pixel {
                    let u = (i as f32 + common::random_float()) / (render.width - 1) as f32;
                    let v = (j as f32 + common::random_float()) / (height - 1) as f32;
                    let r = self.get_ray(u, v);
                    pixel_colour += self.ray_colour(&r, world, render.max_depth);
                }
                colour::get_output_colour(pixel_colour, render.samples_per_pixel)
            })
            .collect::<Vec<(u8, u8, u8)>>();

        // Render
        println!("P3");
        println!("{} {}", render.width, height);
        println!("255");
        for (r, g, b) in pixels {
            println!("{} {} {}", r, g, b);
        }
    }

    fn get_ray(&self, s: f32, t: f32) -> Ray {
        // Construct a camera ray originating from the defocus disk and
        // directed at a randomly sampled point around the pixel location.
        let rd = self.lens_radius * vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let ray_time = common::random_float();

        Ray::new_at(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            ray_time,
        )
    }

    fn ray_colour(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Colour {
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        match world.hit(r, Interval::new(0.001, common::INFINITY)) {
            Some(hit_record) => {
                let emission_colour = hit_record.material.emitted(hit_record.u, hit_record.v, hit_record.p);
                match hit_record.material.scatter(r, &hit_record) {
                    Some((attenuation, scattered)) => {
                        let scatter_colour =
                            attenuation * self.ray_colour(&scattered, world, depth - 1);
                        emission_colour + scatter_colour
                    }
                    None => emission_colour,
                }
            }
            None => self.background,
        }
    }
}
