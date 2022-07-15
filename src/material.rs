use crate::hittable::*;
use crate::ray::*;
use crate::color::*;
use crate::utilities::rand_double;
use crate::vec3::Vec3;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::random_unit_vector;

pub trait Material {
    fn scatter(&self, r_in: &Ray<f64>, rec: &HitRecord<f64>, attenuation: &mut Color<f64>, scattered: &mut Ray<f64>) -> bool;
}

pub struct Lambertian {
    pub albedo: Color<f64>
}

impl Lambertian {
    pub fn new(a: &Color<f64>) -> Self {
        Self{albedo: *a}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray<f64>, rec: &HitRecord<f64>, attenuation: &mut Color<f64>, scattered: &mut Ray<f64>) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    pub albedo: Color<f64>,
    pub fuzz: f64
}

impl Metal {
    pub fn new(a: &Color<f64>, f: f64) -> Self {
        Self{albedo: *a, fuzz: f}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray<f64>, rec: &HitRecord<f64>, attenuation: &mut Color<f64>, scattered: &mut Ray<f64>) -> bool {
        let reflected = r_in.direction.unit_vector().reflect(rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction.dot(rec.normal) > 0.0
    }
}



pub struct Dielectric {
    pub index_of_refraction: f64
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self{index_of_refraction: index_of_refraction}
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray<f64>, rec: &HitRecord<f64>, attenuation: &mut Color<f64>, scattered: &mut Ray<f64>) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.index_of_refraction} else { self.index_of_refraction};
        
        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand_double() {
            direction = unit_direction.reflect(rec.normal)
        } else {
            direction = unit_direction.refract(rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / ( 1.0 + ref_idx);
    let r0 = r0*r0;
    r0 + (1.0 - r0) * (1.0  - cosine).powi(5)
}

