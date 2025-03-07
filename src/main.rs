use ray_tracing::*;
use std::env;

const USAGE: &str = "Usage: ./ray-tracer <scene num> <LOW|MED|HIGH quality>";

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
        3 => earth(&render),
        4 => perlin_spheres(&render),
        5 => quads(&render),
        6 => simple_light(&render),
        7 => cornell_box(&render),
        8 => cornell_smoke(&render),
        _ => panic!("Unsupported scene: {}", scene),
    }
}
