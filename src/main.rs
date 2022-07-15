use raychasing::color::*;
use raychasing::vec3::*;
use raychasing::ray::*;
use raychasing::constants::*;
use raychasing::utilities::*;
use raychasing::hittable_list::*;
use raychasing::hittable::*;
use raychasing::sphere::*;
use raychasing::camera::*;
use raychasing::material::*;

use std::io::{self, Write};
use std::sync::Arc;

fn ray_color<'a>(r: Ray<f64>, world: &'a HittableList, depth: i32) -> Color<f64> 
{
    let mut rec = HitRecord::zero();

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::zero();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::x_unit();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        if rec.mat_ptr.as_ref().scatter( &r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::zero();
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList<'static> {
    let mut world = HittableList::<'static>::new();

    let ground_material = Arc::new(Lambertian::new(
        &Color::new(0.5, 0.5, 0.5)
     ));

     world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

     for a in -2..2 {
        for b in -2..2 {
            let choose_mat = rand_double();
            let center = Point3::new(a as f64 + 0.9 * rand_double(), 0.2, b as f64 + 0.9 * rand_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::rand() * Color::rand();
                    let spherical_material = Arc::new(Lambertian::new(&albedo));
                    world.add(Sphere::new(center, 0.2, spherical_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let spherical_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, spherical_material));
                } else {
                    // glass
                    let spherical_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(center, 0.2, spherical_material));
                }
            }
        }
     };

     let material1 = Arc::new(Dielectric::new(1.5));
     world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

     let material2 = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
     world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

     let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
     world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

     world
}


fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 200;
    let max_depth = 50;

    // World

    let world = random_scene();


    // Camera

    let lookfrom = Point3::new(11.0, 4.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.05; // 2.0;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    // Render

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        io::stderr().write_all(format!("\rScanlines remaining: {}", j).as_bytes())?;
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand_double()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rand_double()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(&mut io::stdout(), pixel_color, samples_per_pixel)?;
        }
    }
    io::stderr().write_all(format!("\nDone.\n").as_bytes())?;

    Ok(())
}
