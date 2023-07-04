use crate::{
    hittable::HitRecord,
    ray::Ray,
    rtweekend::random_double,
    vec3::{
        dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color, Vec3,
    },
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

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.set(&Color::new(1.0, 1.0, 1.0));
        let refraction_ratio = if rec.front_face {
            (1.0 / self.ir)
        } else {
            self.ir
        };
        let unit_direction = unit_vector(&r_in.direction());

        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        scattered.set(&Ray::new(&rec.p, &direction));
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Schlick's apporximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
