#![allow(warnings)]
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use hittable::*;
use hittable_list::*;
use ray::*;
use rtweekend::INFINITY;
use sphere::*;
use std::rc::Rc;
use vec3::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn ray_color(r: &Ray, world: &impl Hittable) -> color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color::new(1.0, 1.0, 1.0) + t * color::new(0.5, 0.7, 1.0)
}

fn main() {
    // World

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(point3::new(0.0, -100.5, 1.0), 100.0)));
    // Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone()
        - horizontal.clone() / 2.
        - vertical.clone() / 2.
        - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {j} ");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH as f64 - 1.);
            let v = j as f64 / (IMAGE_HEIGHT as f64 - 1.);
            let dir = lower_left_corner.clone() + u * horizontal.clone() + v * vertical.clone()
                - origin.clone();
            let r = Ray::new(&origin, &dir);
            let pixel_color = ray_color(&r, &world);
            write_color(&pixel_color);
        }
    }

    eprintln!("Done!")
}
