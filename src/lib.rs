#![allow(dead_code)]

mod aabb;
mod bvh_node;
mod camera;
mod colour;
mod common;
mod constant_medium;
mod hittable;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rotate_y;
pub mod sphere;
mod texture;
mod translate;
pub mod vec3;

use bvh_node::Bvh;
use camera::{Camera, Render};
use colour::Colour;
use constant_medium::ConstantMedium;
use hittable::Hittable;
use material::Material;
use perlin::Perlin;
use quad::{Quad, quad_box};
use rotate_y::RotateY;
use sphere::Sphere;
use texture::Texture;
use translate::Translate;
use vec3::{Point3, Vec3};

// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const SQUARE_ASPECT_RATIO: f32 = 1.0;

pub const LOWLOW_RENDER: Render = Render {
    width: 400,
    samples_per_pixel: 5,
    max_depth: 4,
};

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

pub const FINAL_RENDER: Render = Render {
    width: 800,
    samples_per_pixel: 10000,
    max_depth: 40,
};

pub fn bouncing_spheres(render: &Render) {
    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let ground_material = Material::Lambertian {
        texture: Texture::Checker {
            scale: Vec3::fill(0.32),
            even: Box::new(Colour::new(0.2, 0.3, 0.1).into()),
            odd: Box::new(Colour::new(0.9, 0.9, 0.9).into()),
        },
    };

    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = common::random_float();
            let center = Point3::new(
                a as f32 + 0.9 * common::random_float(),
                0.2,
                b as f32 + 0.9 * common::random_float(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Colour::random() * Colour::random();
                    let sphere_material = Material::Lambertian {
                        texture: albedo.into(),
                    };
                    let centre2 =
                        center + Point3::new(0.0, common::random_float_range(0.0, 0.5), 0.0);
                    world.push(Box::new(Sphere::moving(
                        (center, centre2),
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // Meta
                    let albedo = Colour::random_range(0.5, 1.0);
                    let fuzziness = common::random_float_range(0.0, 0.5);
                    let sphere_material = Material::Metal { albedo, fuzziness };
                    world.push(Box::new(Sphere::stationary(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Material::Dielectric { refraction: 1.5 };
                    world.push(Box::new(Sphere::stationary(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material = Material::Dielectric { refraction: 1.5 };
    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));
    let material = Material::Dielectric {
        refraction: 1.0 / 1.5,
    };
    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        0.9,
        material,
    )));

    let material = Material::Lambertian {
        texture: Colour::new(0.4, 0.2, 0.1).into(),
    };
    world.push(Box::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Material::Metal {
        albedo: Colour::new(0.7, 0.6, 0.5),
        fuzziness: 0.0,
    };
    world.push(Box::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));
    let world = Bvh::new(world);

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
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let checker = Texture::Checker {
        scale: Vec3::fill(0.32),
        even: Box::new(Colour::new(0.2, 0.3, 0.1).into()),
        odd: Box::new(Colour::new(0.9, 0.9, 0.9).into()),
    };

    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Material::Lambertian {
            texture: checker.clone(),
        },
    )));

    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Material::Lambertian {
            texture: checker.clone(),
        },
    )));
    let world = Bvh::new(world);

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
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let earth_texture = Texture::Image {
        image: texture::load_image("earthmap.jpg"),
    };
    let earth_surface = Material::Lambertian {
        texture: earth_texture,
    };

    world.push(Box::new(Sphere::stationary(
        Point3::zero(),
        2.0,
        earth_surface,
    )));
    let world = Bvh::new(world);

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
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let perlin_texture = Texture::Noise {
        noise: Box::new(Perlin::new()),
        scale: 4.0,
    };

    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            texture: perlin_texture.clone(),
        },
    )));

    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Material::Lambertian {
            texture: perlin_texture.clone(),
        },
    )));
    let world = Bvh::new(world);

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
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

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
    world.push(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));
    let world = Bvh::new(world);

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
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let perlin_texture = Texture::Noise {
        noise: Box::new(Perlin::new()),
        scale: 4.0,
    };

    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            texture: perlin_texture.clone(),
        },
    )));

    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Material::Lambertian {
            texture: perlin_texture.clone(),
        },
    )));

    let difflight = Material::DiffuseLight {
        texture: Colour::new(4.0, 4.0, 4.0).into(),
    };
    world.push(Box::new(Sphere::stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));
    let world = Bvh::new(world);

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
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

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

    world.push(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.push(Box::new(Quad::new(
        Point3::zero(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.push(Box::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    world.push(Box::new(Quad::new(
        Point3::zero(),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.push(Box::new(Quad::new(
        Point3::fill(555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = quad_box(Point3::zero(), Point3::newi(165, 330, 165), white.clone());
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::newi(265, 0, 200));
    world.push(Box::new(box1));

    let box2 = quad_box(Point3::zero(), Point3::fill(165.0), white.clone());
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::newi(130, 0, 65));
    world.push(Box::new(box2));
    let world = Bvh::new(world);

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

pub fn cornell_smoke(render: &Render) {
    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

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
        texture: Colour::fill(7.0).into(),
    };

    world.push(Box::new(Quad::new(
        Point3::newi(555, 0, 0),
        Vec3::newi(0, 555, 0),
        Vec3::newi(0, 0, 555),
        green,
    )));

    world.push(Box::new(Quad::new(
        Point3::zero(),
        Vec3::newi(0, 555, 0),
        Vec3::newi(0, 0, 555),
        red,
    )));

    world.push(Box::new(Quad::new(
        Point3::newi(113, 554, 127),
        Vec3::newi(330, 0, 0),
        Vec3::newi(0, 0, 305),
        light.clone(),
    )));

    world.push(Box::new(Quad::new(
        Point3::newi(0, 555, 0),
        Vec3::newi(555, 0, 0),
        Vec3::newi(0, 0, 555),
        white.clone(),
    )));

    world.push(Box::new(Quad::new(
        Point3::zero(),
        Vec3::newi(555, 0, 0),
        Vec3::newi(0, 0, 555),
        white.clone(),
    )));

    world.push(Box::new(Quad::new(
        Point3::newi(0, 0, 555),
        Vec3::newi(555, 0, 0),
        Vec3::newi(0, 555, 0),
        white.clone(),
    )));

    let box1 = quad_box(Point3::zero(), Point3::newi(165, 330, 165), white.clone());
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::newi(265, 0, 295));
    world.push(Box::new(ConstantMedium::new(
        box1,
        0.01,
        Colour::fill(0.0).into(),
    )));

    let box2 = quad_box(Point3::zero(), Point3::fill(165.0), white.clone());
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::newi(130, 0, 65));
    world.push(Box::new(ConstantMedium::new(
        box2,
        0.01,
        Colour::fill(1.0).into(),
    )));

    let world = Bvh::new(world);

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

pub fn final_scene(render: &Render) {
    let ground = Material::Lambertian {
        texture: Colour::new(0.48, 0.83, 0.53).into(),
    };

    let mut boxes1: Vec<Box<dyn Hittable>> = Vec::new();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = common::random_float_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Box::new(quad_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Bvh::new(boxes1)));

    let light = Material::DiffuseLight {
        texture: Colour::fill(7.0).into(),
    };
    world.push(Box::new(Quad::new(
        Point3::newi(123, 553, 147),
        Vec3::newi(300, 0, 0),
        Vec3::newi(0, 0, 265),
        light.clone(),
    )));

    let center1 = Point3::newi(400, 400, 200);
    let center2 = center1 + Vec3::newi(30, 0, 0);
    let sphere_material = Material::Lambertian {
        texture: Colour::new(0.7, 0.3, 0.1).into(),
    };
    world.push(Box::new(Sphere::moving(
        (center1, center2),
        50.0,
        sphere_material,
    )));

    world.push(Box::new(Sphere::stationary(
        Point3::newi(260, 150, 45),
        50.0,
        Material::Dielectric { refraction: 1.5 },
    )));
    world.push(Box::new(Sphere::stationary(
        Point3::newi(0, 150, 145),
        50.0,
        Material::Metal {
            albedo: Colour::new(0.8, 0.8, 0.9),
            fuzziness: 1.0,
        },
    )));

    world.push(Box::new(Sphere::stationary(
        Point3::newi(360, 150, 145),
        70.0,
        Material::Dielectric { refraction: 1.5 },
    )));
    world.push(Box::new(ConstantMedium::new(
        Sphere::stationary(
            Point3::newi(360, 150, 145),
            70.0,
            Material::Dielectric { refraction: 1.5 },
        ),
        0.2,
        Colour::new(0.2, 0.4, 0.9).into(),
    )));
    let boundary = Sphere::stationary(
        Point3::zero(),
        5000.0,
        Material::Dielectric { refraction: 1.5 },
    );
    world.push(Box::new(ConstantMedium::new(
        boundary,
        0.0001,
        Colour::fill(1.0).into(),
    )));

    let emat = Material::Lambertian {
        texture: Texture::Image {
            image: texture::load_image("earthmap.jpg"),
        },
    };
    world.push(Box::new(Sphere::stationary(
        Point3::newi(400, 200, 400),
        100.0,
        emat,
    )));
    let perlin_texture = Texture::Noise {
        noise: Box::new(Perlin::new()),
        scale: 4.0,
    };
    world.push(Box::new(Sphere::stationary(
        Point3::newi(220, 280, 300),
        80.0,
        Material::Lambertian {
            texture: perlin_texture,
        },
    )));

    let mut boxes2: Vec<Box<dyn Hittable>> = Vec::new();
    let white = Material::Lambertian {
        texture: Colour::new(0.73, 0.73, 0.73).into(),
    };
    let ns = 1000;
    for _ in 0..ns {
        boxes2.push(Box::new(Sphere::stationary(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    let boxes2 = RotateY::new(Bvh::new(boxes2), 15.0);
    let boxes2 = Translate::new(boxes2, Vec3::newi(-100, 270, 395));
    world.push(Box::new(boxes2));

    let world = Bvh::new(world);

    // Camera
    let cam = Camera::new(
        Point3::newi(478, 278, -600),
        Point3::newi(278, 278, 0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        SQUARE_ASPECT_RATIO,
        0.1,
        (Point3::new(478.0, 278.0, -600.0) - Point3::new(278.0, 278.0, 0.0)).length(),
        Colour::zero(),
    );

    cam.render(&world, render);
}
