use crate::common;
use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn get_output_colour(pixel_colour: Colour, samples_per_pixel: i32) -> (u8, u8, u8) {
    let mut r = pixel_colour.x();
    let mut g = pixel_colour.y();
    let mut b = pixel_colour.z();

    // Divide the colour by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    // Write the translated [0, 255] value of each colour component.
    (
        (256.0 * common::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * common::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * common::clamp(b, 0.0, 0.999)) as u8,
    )
}
