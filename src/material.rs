use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Color},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Lambertian {
            albedo: Color::new(a[0], a[1], a[2]),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        scattered.set(&Ray::new(&rec.p, &scatter_direction));
        attenuation.set(&self.albedo);
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: &Color, fuzz: f64) -> Self {
        Metal {
            albedo: Color::new(a[0], a[1], a[2]),
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal);
        scattered.set(&Ray::new(
            &rec.p,
            &(reflected + self.fuzz * random_in_unit_sphere()),
        ));
        attenuation.set(&self.albedo);
        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
