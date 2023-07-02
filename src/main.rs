#![allow(warnings)]
mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use camera::*;
use hittable::*;
use hittable_list::*;
use ray::*;
use rtweekend::INFINITY;
use sphere::*;
use std::rc::Rc;
use vec3::*;

use crate::rtweekend::random_double;

fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::new();

    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.clone().p + rec.clone().normal + random_unit_vector();
        return 0.5
            * ray_color(
                &Ray::new(&rec.clone().p, &(target - rec.clone().p)),
                world,
                depth - 1,
            );
    }
    let unit_direction = unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let depth = 50;
    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, 1.0), 100.0)));

    // Camera
    let cam = Camera::new();
    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {j} ");
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64 - 1.);
                let v = (j as f64 + random_double()) / (image_height as f64 - 1.);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, depth);
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done!")
}
