pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod material;

pub mod constants {
    pub const INFINITY: f64 = std::f64::INFINITY;
    pub const PI: f64 = 3.1415926535897932385;
}

pub mod utilities {
    extern crate rand;

    use super::constants::*;
    use rand::prelude::*;

    #[inline]
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    pub fn rand_double() -> f64 {
        let mut rng = rand::thread_rng();
        let r = rng.gen();
        r
    }

    pub fn random_double(min: f64, max: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let r: f64 = rng.gen();
        min + (max - min) * r
    }

    #[inline]
    pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn output_ppm_test() {
    }
}
