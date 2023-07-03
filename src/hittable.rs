use crate::material::{Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::vec3::{dot, Color, Point3, Vec3};
use std::rc::Rc;
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Rc<dyn Material>,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            material: Rc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0))),
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }

    pub fn set(&mut self, other: &HitRecord) {
        self.p = other.p;
        self.normal = other.normal;
        self.t = other.t;
        self.material = other.material.clone();
        self.front_face = other.front_face
    }
}
