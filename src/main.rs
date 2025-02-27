mod camera;
mod colour;
mod common;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use colour::Colour;
use hittable::Hittable;
use hittable_list::HittableList;
use interval::Interval;
use material::Material;
use ray::Ray;
use sphere::Sphere;
use std::sync::Mutex;
use vec3::Point3;

use rayon::prelude::*;

fn ray_colour(r: &Ray, world: &dyn Hittable, depth: i32) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, Interval::new(0.001, common::INFINITY)) {
        Some((hit_record, material)) => match material.scatter(r, &hit_record) {
            Some((attenuation, scattered)) => {
                attenuation * ray_colour(&scattered, world, depth - 1)
            }
            None => Colour::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian {
        albedo: Colour::new(0.5, 0.5, 0.5),
    };
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = common::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * common::random_double(),
                0.2,
                b as f64 + 0.9 * common::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Colour::random() * Colour::random();
                    let sphere_material = Material::Lambertian { albedo };
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Colour::random_range(0.5, 1.0);
                    let fuzziness = common::random_double_range(0.0, 0.5);
                    let sphere_material = Material::Metal { albedo, fuzziness };
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Material::Dialectric { refraction: 1.5 };
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material = Material::Dialectric { refraction: 1.5 };
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));
    let material = Material::Dialectric {
        refraction: 1.0 / 1.5,
    };
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        0.9,
        material,
    )));

    let material = Material::Lambertian {
        albedo: Colour::new(0.4, 0.2, 0.1),
    };
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Material::Metal {
        albedo: Colour::new(0.7, 0.6, 0.5),
        fuzziness: 0.0,
    };
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 5; // 500; // 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let progress = Mutex::new(0);

    let num_pixels = IMAGE_WIDTH * IMAGE_HEIGHT;

    let pixels: Vec<(u8, u8, u8)> = (0..num_pixels)
        .into_par_iter()
        .map(|index| {
            {
                let mut count = progress.lock().unwrap();
                if *count % 1000 == 0 {
                    eprint!("\rScanlines remaining: {}", (num_pixels - *count));
                }
                *count += 1;
            }

            let i = index % IMAGE_WIDTH;
            let j = IMAGE_HEIGHT - (index / IMAGE_WIDTH) - 1;

            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world, MAX_DEPTH);
            }
            colour::get_output_colour(pixel_colour, SAMPLES_PER_PIXEL)
        })
        .collect::<Vec<(u8, u8, u8)>>();

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    for (r, g, b) in pixels {
        println!("{} {} {}", r, g, b);
    }
}
