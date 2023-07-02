use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, r: f64) -> Self {
        Sphere { center, radius: r }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center.clone();
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt = discriminant.sqrt();
        let mut root = (-half_b - sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p.clone() - self.center.clone()) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        true
    }
}
