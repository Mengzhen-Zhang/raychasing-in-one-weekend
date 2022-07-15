use crate::utilities::degrees_to_radians;
use crate::vec3::*;
use crate::ray::*;

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>,
    lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom: Point3<f64>, lookat: Point3<f64>, vup: Vec3<f64>, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let veiwport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * veiwport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;

        let len_radius = aperture / 2.0;

        Camera { origin: origin, lower_left_corner: lower_left_corner, horizontal: horizontal , vertical: vertical, u: u, v: v, w: w, lens_radius: len_radius }
    }

    pub fn get_ray(&self, s: f64, t:f64) -> Ray<f64> {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin - offset;
        Ray::new(origin, direction)
    }
}