use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + t * self.dir.clone()
    }

    pub fn set(&mut self, other: &Ray) {
        self.orig = other.clone().orig;
        self.dir = other.clone().dir;
    }
}
