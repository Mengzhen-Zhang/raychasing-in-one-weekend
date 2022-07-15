use std::io::{Write, self};

use crate::vec3::{Vec3};
use crate::utilities::*;

pub type Color<T> = Vec3<T>;

pub fn write_color<T>(output: &mut T, pixel_color: Color<f64>, samples_per_pixel: i32) -> io::Result<()>
where
    T: Write
{
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    // Write the translated [0, 255] value of each color component.
    let ir = (255.999 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (255.999 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (255.999 * clamp(b, 0.0, 0.999)) as i32;
    output.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
    Ok(())
}