mod camera;
mod colour;
mod common;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::io;
use std::rc::Rc;

use camera::Camera;
use colour::Colour;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::Lambertian;
use ray::Ray;
use sphere::Sphere;
use vec3::Point3;

fn ray_colour(r: &Ray, world: &dyn Hittable, depth: i32) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let mut attenuation = Colour::default();
        let mut scattered = Ray::default();
        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_colour(&scattered, world, depth - 1);
        }
        return Colour::new(0.0, 0.0, 0.0);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 800;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let r = f64::cos(common::PI / 4.0);
    let mut world = HittableList::new();

    let material_left = Rc::new(Lambertian::new(Colour::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Colour::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    // Camera
    let cam = Camera::new(90.0, ASPECT_RATIO);

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j: >5}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world, MAX_DEPTH);
            }
            colour::write_colour(&mut io::stdout(), pixel_colour, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.")
}
