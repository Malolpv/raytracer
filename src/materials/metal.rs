use crate::{color::Color, materials::Material, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1_f64),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &crate::components::HitRecord,
    ) -> Option<(crate::ray::Ray, Color)> {
        let mut reflected = Vec3::reflect(ray_in.direction(), rec.normal());
        reflected = Vec3::unit_vector(&reflected) + (Vec3::random_unit_vector() * self.fuzz);

        let scattered = Ray::new(rec.position(), reflected);
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction(), &rec.normal()) > 0_f64 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
