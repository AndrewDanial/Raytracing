#![allow(warnings)]
const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
mod vec3;
use vec3::*;
fn main() {
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {j} ");
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Vec3::new(
                i as f64 / (IMAGE_WIDTH as f64 - 1.),
                j as f64 / (IMAGE_HEIGHT as f64 - 1.),
                0.25,
            );
            write_color(pixel_color);
        }
    }

    eprintln!("Done!")
}
