use std::sync::Arc;

use crate::color::Color;
use crate::vec3::{Vec3, Point3};
use crate::ray::*;
use crate::material::*;


#[derive(Clone)]
pub struct HitRecord<T> {
    pub p: Point3<T>,
    pub normal: Vec3<T>,
    pub t: T,
    pub front_face: bool,
    pub mat_ptr: Arc< dyn Material>
}

impl<'a> HitRecord<f64>
{
    pub fn new(p: Point3<f64>, normal: Vec3<f64>, t: f64, front_face: bool, mat_ptr: Arc<dyn Material>) -> Self { Self { p, normal, t, front_face , mat_ptr} }

    pub fn zero() -> Self {Self::new(Vec3::<f64>::zero(), Vec3::<f64>::zero(), 0.0, false, Arc::new(Lambertian{albedo: Color::new(0.0, 0.0, 0.0)}))}

    pub fn set_face_normal(&mut self, r: &Ray<f64>, outward_normal: &Vec3<f64>) {
        self.front_face = r.direction.dot(*outward_normal) < 0.;
        self.normal = if self.front_face {*outward_normal} else {- *outward_normal};
    }
}

pub trait Hittable
{
    fn hit(&self, r: Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord<f64>) -> bool;
}
