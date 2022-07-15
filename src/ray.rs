use crate::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray<T> {
    pub origin: Point3<T>,
    pub direction: Vec3<T>
}

impl Ray<f64> {
    pub fn new(origin: Point3<f64>, direction: Vec3<f64>) -> Ray<f64> {
        Ray { origin: origin, direction: direction }
    }

    pub fn x_unit() -> Ray<f64> {
        Ray::new(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0))
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }
}