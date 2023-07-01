use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, mut rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.clone() {
            let mut temp = temp_rec.clone();
            if object.hit(r, t_min, closest_so_far, &mut temp) {
                hit_anything = true;
                closest_so_far = temp.clone().t;
                rec.set(temp);
            }
        }

        return hit_anything;
    }
}
