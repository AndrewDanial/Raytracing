use std::{io::Write, ops};

use crate::rtweekend::{random_double, random_double_range};

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    e: Vec<f64>,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            e: vec![e0, e1, e2],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn random() -> Vec3 {
        Vec3 {
            e: vec![random_double(), random_double(), random_double()],
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: vec![
                random_double_range(min, max),
                random_double_range(min, max),
                random_double_range(min, max),
            ],
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        (1. / rhs) * self
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: vec![-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v.clone() / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if (dot(&in_unit_sphere, normal) > 0.0) {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn write_color(pixel_color: &Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    // Divide color by number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    // Write the translated [0, 255] value of each color component
    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;
    let mut out = std::io::stdout();
    out.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))
        .unwrap();
}
