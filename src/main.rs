mod colour;
mod ray;
mod vec3;

use std::io;

use colour::Color;

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            eprint!("\rScanlines remaining: {}", j);
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.25;

            let pixel_colour = Color::new(r, g, b);
            colour::write_colour(&mut io::stdout(), pixel_colour);
        }
    }

    eprintln!("\nDone.")
}
