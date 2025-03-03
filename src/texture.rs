use crate::colour::Colour;
use crate::perlin::Perlin;
use crate::vec3::{Point3, Vec3};

use image::{Pixel, RgbImage};

pub fn load_image(name: &str) -> RgbImage {
    let img = image::open(format!(
        "{}/{}",
        "/Users/sam/Projects/rust/ray-tracing/data/", name
    ))
    .unwrap();
    img.to_rgb8()
}

#[derive(Clone)]
pub enum Texture {
    SolidColour {
        albedo: Colour,
    },
    Checker {
        scale: Vec3,
        even: Box<Texture>,
        odd: Box<Texture>,
    },
    Image {
        image: RgbImage,
    },
    Noise {
        noise: Box<Perlin>,
        scale: f64,
    },
}

impl From<Colour> for Texture {
    fn from(colour: Colour) -> Texture {
        Texture::SolidColour { albedo: colour }
    }
}

impl Texture {
    pub fn colour(&self, u: f64, v: f64, p: Point3) -> Colour {
        match self {
            Texture::SolidColour { albedo } => *albedo,
            Texture::Checker { scale, even, odd } => {
                let inv_scale = 1.0 / *scale;
                let x = f64::floor(inv_scale.x() * p.x()) as i32;
                let y = f64::floor(inv_scale.y() * p.y()) as i32;
                let z = f64::floor(inv_scale.z() * p.z()) as i32;

                let is_even = (x + y + z) % 2 == 0;

                if is_even {
                    even.colour(u, v, p)
                } else {
                    odd.colour(u, v, p)
                }
            }
            Texture::Image { image } => {
                // If we have no texture data, then return solid cyan as a debugging aid.
                if image.height() == 0 {
                    return Colour::new(0.0, 1.0, 1.0);
                };

                let u = u.clamp(0.0, 1.0);
                let v = v.clamp(0.0, 1.0);

                let mut i = f64::floor(u * image.width() as f64) as u32;
                i %= image.width();

                let mut j = f64::floor(v * image.height() as f64) as u32;
                j %= image.height();
                // Invert j
                j = image.height() - j;

                let pixel = image.get_pixel(i, j).to_rgb();
                let colour_scale = 1.0 / 255.0;

                Colour::new(
                    colour_scale * pixel.0[0] as f64,
                    colour_scale * pixel.0[1] as f64,
                    colour_scale * pixel.0[2] as f64,
                )
            }
            Texture::Noise { noise, scale } => {
                Colour::fill(0.5) * (1.0 + f64::sin(scale * p.z() + 10.0 * noise.turb(p, 7)))
            }
        }
    }
}
