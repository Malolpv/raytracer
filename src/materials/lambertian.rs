use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{color::Color, materials::Material};

pub struct Lambertian {
    /// Latin for whiteness, define some form of fractional reflectance
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &crate::ray::Ray,
        rec: &crate::components::HitRecord,
    ) -> Option<(crate::ray::Ray, crate::color::Color)> {
        let mut scatter_direction = rec.normal() + Vec3::random_unit_vector();

        // Catch degenerate scatter directions
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        let ray_out = Ray::new(rec.position(), scatter_direction);
        let attenuation = self.albedo;
        Some((ray_out, attenuation))
    }
}
