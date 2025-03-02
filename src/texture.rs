use crate::colour::Colour;
use crate::vec3::Point3;

#[derive(Clone)]
pub enum Texture {
    SolidColour {
        albedo: Colour,
    },
    CheckerTexture {
        scale: f64,
        even: Box<Texture>,
        odd: Box<Texture>,
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
            Texture::CheckerTexture { scale, even, odd } => {
                let inv_scale = 1.0 / scale;
                let x = f64::floor(inv_scale * p.x()) as i32;
                let y = f64::floor(inv_scale * p.y()) as i32;
                let z = f64::floor(inv_scale * p.z()) as i32;

                let is_even = (x + y + z) % 2 == 0;

                if is_even {
                    even.colour(u, v, p)
                } else {
                    odd.colour(u, v, p)
                }
            }
        }
    }
}
