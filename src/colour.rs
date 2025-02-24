use std::io::Write;

use crate::common;
use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_colour(out: &mut impl Write, pixel_colour: Colour, samples_per_pixel: i32) {
    let mut r = pixel_colour.x();
    let mut g = pixel_colour.y();
    let mut b = pixel_colour.z();

    // Divide the colour by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // Write the translated [0, 255] value of each colour component.
    writeln!(
        out,
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("Writing colour");
}
