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

use camera::{Camera, Render};
use colour::Colour;
use hittable_list::HittableList;
use material::Material;
use sphere::Sphere;
use vec3::Point3;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian {
        albedo: Colour::new(0.5, 0.5, 0.5),
    };
    world.add(Box::new(Sphere::stationary(
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
                    let centre2 =
                        center + Point3::new(0.0, common::random_double_range(0.0, 0.5), 0.0);
                    world.add(Box::new(Sphere::moving(
                        (center, centre2),
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Colour::random_range(0.5, 1.0);
                    let fuzziness = common::random_double_range(0.0, 0.5);
                    let sphere_material = Material::Metal { albedo, fuzziness };
                    world.add(Box::new(Sphere::stationary(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Material::Dialectric { refraction: 1.5 };
                    world.add(Box::new(Sphere::stationary(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material = Material::Dialectric { refraction: 1.5 };
    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));
    let material = Material::Dialectric {
        refraction: 1.0 / 1.5,
    };
    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        0.9,
        material,
    )));

    let material = Material::Lambertian {
        albedo: Colour::new(0.4, 0.2, 0.1),
    };
    world.add(Box::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Material::Metal {
        albedo: Colour::new(0.7, 0.6, 0.5),
        fuzziness: 0.0,
    };
    world.add(Box::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0; //3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 400; // 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100; // 500;
    const MAX_DEPTH: i32 = 50;

    let render = Render {
        width: IMAGE_WIDTH,
        height: IMAGE_HEIGHT,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
    };

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

    cam.render(&world, &render);
}
