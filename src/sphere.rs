use std::sync::Arc;

use crate::hittable::*;
use crate::vec3::*;
use crate::material::*;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>
}

impl<'a> Sphere {
    pub fn new(center: Point3<f64>, radius: f64, mat_ptr: Arc<dyn Material>) -> Self { Self { center, radius, mat_ptr } }
}

impl Hittable for Sphere {
    fn hit(&self, r: crate::ray::Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord<f64>) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {return false};
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (- half_b - sqrtd ) / a;
        let root = if root < t_min || t_max < root { (-half_b + sqrtd) / a } else {root};
        if root < t_min || t_max < root { return false; };

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}

