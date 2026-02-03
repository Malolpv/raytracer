use crate::{color::Color, materials::Material, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &crate::components::HitRecord,
    ) -> Option<(crate::ray::Ray, Color)> {
        let reflected = Vec3::reflect(ray_in.direction(), rec.normal());

        let ray_out = Ray::new(rec.position(), reflected);
        let attenuation = self.albedo;

        Some((ray_out, attenuation))
    }
}
