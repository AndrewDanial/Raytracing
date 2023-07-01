use crate::vec3::{color, point3, Vec3};

pub struct Ray {
    orig: point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub fn origin(&self) -> point3 {
        self.orig.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    pub fn at(&self, t: f64) -> point3 {
        self.orig.clone() + t * self.dir.clone()
    }
}
