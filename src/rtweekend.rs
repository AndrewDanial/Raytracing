use rand::prelude::*;
pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    y
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
