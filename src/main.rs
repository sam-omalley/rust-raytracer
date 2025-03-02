#![allow(dead_code)]

mod aabb;
mod bvh_node;
mod camera;
mod colour;
mod common;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod texture;
mod vec3;

use bvh_node::BvhNode;
use camera::{Camera, Render};
use colour::Colour;
use hittable_list::HittableList;
use material::Material;
use sphere::Sphere;
use texture::Texture;
use vec3::{Point3, Vec3};

use std::env;
use std::sync::Arc;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const LOW_QUALITY_RENDER: Render = Render {
    width: 640,
    height: 360,
    samples_per_pixel: 50,
    max_depth: 50,
};

const MEDIUM_QUALITY_RENDER: Render = Render {
    width: 1820,
    height: 1024,
    samples_per_pixel: 100,
    max_depth: 50,
};

const HIGH_QUALITY_RENDER: Render = Render {
    width: 1920,
    height: 1080,
    samples_per_pixel: 500,
    max_depth: 50,
};

fn bouncing_spheres(render: &Render) {
    // World
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian {
        texture: Texture::CheckerTexture {
            scale: 0.32,
            even: Box::new(Colour::new(0.2, 0.3, 0.1).into()),
            odd: Box::new(Colour::new(0.9, 0.9, 0.9).into()),
        },
    };

    world.add(Arc::new(Sphere::stationary(
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
                    let sphere_material = Material::Lambertian {
                        texture: albedo.into(),
                    };
                    let centre2 =
                        center + Point3::new(0.0, common::random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::moving(
                        (center, centre2),
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Colour::random_range(0.5, 1.0);
                    let fuzziness = common::random_double_range(0.0, 0.5);
                    let sphere_material = Material::Metal { albedo, fuzziness };
                    world.add(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Material::Dialectric { refraction: 1.5 };
                    world.add(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material = Material::Dialectric { refraction: 1.5 };
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));
    let material = Material::Dialectric {
        refraction: 1.0 / 1.5,
    };
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        0.9,
        material,
    )));

    let material = Material::Lambertian {
        texture: Colour::new(0.4, 0.2, 0.1).into(),
    };
    world.add(Arc::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Material::Metal {
        albedo: Colour::new(0.7, 0.6, 0.5),
        fuzziness: 0.0,
    };
    world.add(Arc::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));
    let world = BvhNode::new(&world.objects());

    // Camera
    let cam = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    cam.render(&world, render);
}

fn checkered_spheres(render: &Render) {
    // World
    let mut world = HittableList::new();

    let checker = Texture::CheckerTexture {
        scale: 0.32,
        even: Box::new(Colour::new(0.2, 0.3, 0.1).into()),
        odd: Box::new(Colour::new(0.9, 0.9, 0.9).into()),
    };

    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Material::Lambertian {
            texture: checker.clone(),
        },
    )));

    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Material::Lambertian {
            texture: checker.clone(),
        },
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(13.0, 2.0, 3.),
        Point3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    cam.render(&world, render);
}

const USAGE: &str = "Usage: ./ray-tracer <scene num> <LOW|MED|HIGH quality>";
enum Quality {
    Low,
    Medium,
    High,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("{}", USAGE)
    }

    let scene: i32 = args[1].parse().unwrap();
    let quality: String = args[2].clone();

    let render = match quality.to_lowercase().as_str() {
        "low" => LOW_QUALITY_RENDER,
        "med" => MEDIUM_QUALITY_RENDER,
        "high" => HIGH_QUALITY_RENDER,
        _ => panic!("{}", USAGE),
    };

    match scene {
        1 => bouncing_spheres(&render),
        2 => checkered_spheres(&render),
        _ => panic!("Unsupported scene: {}", scene),
    }
}
