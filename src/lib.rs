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
mod perlin;
mod quad;
mod ray;
pub mod sphere;
mod texture;
pub mod vec3;

use bvh_node::BvhNode;
use camera::{Camera, Render};
use colour::Colour;
use hittable_list::HittableList;
use material::Material;
use perlin::Perlin;
use quad::Quad;
use sphere::Sphere;
use texture::Texture;
use vec3::{Point3, Vec3};

use std::sync::Arc;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const SQUARE_ASPECT_RATIO: f64 = 1.0;

pub const LOW_QUALITY_RENDER: Render = Render {
    width: 640,
    samples_per_pixel: 50,
    max_depth: 50,
};

pub const MEDIUM_QUALITY_RENDER: Render = Render {
    width: 1820,
    samples_per_pixel: 100,
    max_depth: 50,
};

pub const HIGH_QUALITY_RENDER: Render = Render {
    width: 1920,
    samples_per_pixel: 500,
    max_depth: 50,
};

pub fn bouncing_spheres(render: &Render) {
    // World
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian {
        texture: Texture::Checker {
            scale: Vec3::fill(0.32),
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
        Colour::new(0.7, 0.8, 1.0),
    );

    cam.render(&world, render);
}

pub fn checkered_spheres(render: &Render) {
    // World
    let mut world = HittableList::new();

    let checker = Texture::Checker {
        scale: Vec3::fill(0.32),
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
        Colour::new(0.7, 0.8, 1.0),
    );

    cam.render(&world, render);
}

pub fn earth(render: &Render) {
    // World
    let mut world = HittableList::new();

    let earth_texture = Texture::Image {
        image: texture::load_image("earthmap.jpg"),
    };
    let earth_surface = Material::Lambertian {
        texture: earth_texture,
    };

    world.add(Arc::new(Sphere::stationary(
        Point3::zero(),
        2.0,
        earth_surface,
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(0.0, 0.0, 12.0),
        Point3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
        Colour::new(0.7, 0.8, 1.0),
    );

    cam.render(&world, render);
}

pub fn perlin_spheres(render: &Render) {
    // World
    let mut world = HittableList::new();

    let perlin_texture = Texture::Noise {
        noise: Box::new(Perlin::new()),
        scale: 4.0,
    };

    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            texture: perlin_texture.clone(),
        },
    )));

    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Material::Lambertian {
            texture: perlin_texture.clone(),
        },
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
        Colour::new(0.7, 0.8, 1.0),
    );

    cam.render(&world, render);
}

pub fn quads(render: &Render) {
    // World
    let mut world = HittableList::new();

    // Materials
    let left_red = Material::Lambertian {
        texture: Colour::new(1.0, 0.2, 0.2).into(),
    };
    let back_green = Material::Lambertian {
        texture: Colour::new(0.2, 1.0, 0.2).into(),
    };
    let right_blue = Material::Lambertian {
        texture: Colour::new(0.2, 0.2, 1.0).into(),
    };
    let upper_orange = Material::Lambertian {
        texture: Colour::new(1.0, 0.5, 0.0).into(),
    };
    let lower_teal = Material::Lambertian {
        texture: Colour::new(0.2, 0.8, 0.8).into(),
    };

    // Quads
    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(0.0, 0.0, 9.0),
        Point3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        80.0,
        SQUARE_ASPECT_RATIO,
        0.1,
        10.0,
        Colour::new(0.7, 0.8, 1.0),
    );

    cam.render(&world, render);
}

pub fn simple_light(render: &Render) {
    // World
    let mut world = HittableList::new();

    let perlin_texture = Texture::Noise {
        noise: Box::new(Perlin::new()),
        scale: 4.0,
    };

    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            texture: perlin_texture.clone(),
        },
    )));

    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Material::Lambertian {
            texture: perlin_texture.clone(),
        },
    )));

    let difflight = Material::DiffuseLight {
        texture: Colour::new(4.0, 4.0, 4.0).into(),
    };
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        (Point3::new(26.0, 3.0, 6.0) - Point3::new(0.0, 2.0, 0.0)).length(),
        Colour::zero(),
    );

    cam.render(&world, render);
}

pub fn cornell_box(render: &Render) {
    // World
    let mut world = HittableList::new();

    let red = Material::Lambertian {
        texture: Colour::new(0.65, 0.05, 0.05).into(),
    };
    let white = Material::Lambertian {
        texture: Colour::fill(0.73).into(),
    };
    let green = Material::Lambertian {
        texture: Colour::new(0.12, 0.45, 0.15).into(),
    };
    let light = Material::DiffuseLight {
        texture: Colour::fill(15.0).into(),
    };

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.add(Arc::new(Quad::new(
        Point3::zero(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::zero(),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::fill(555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        SQUARE_ASPECT_RATIO,
        0.1,
        (Point3::new(278.0, 278.0, -800.0) - Point3::new(278.0, 278.0, 0.0)).length(),
        Colour::zero(),
    );

    cam.render(&world, render);
}
