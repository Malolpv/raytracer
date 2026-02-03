use crate::{color::Color, materials::Material, ray::Ray, vec3::Vec3};

pub struct Dielectrics {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over the refractive index of the enclosing media
    ///
    /// ex: to render a glass in air : refractive index of air / refractive index of water
    ///
    /// which equals to => (1.5/1.333)
    refraction_index: f64,
}

impl Dielectrics {
    pub fn new(refraction_index: f64) -> Self {
        Dielectrics { refraction_index }
    }
}

impl Material for Dielectrics {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &crate::components::HitRecord,
    ) -> Option<(crate::ray::Ray, crate::color::Color)> {
        let attenuation: Color = Color::new(1_f64, 1_f64, 1_f64);
        let refraction_index: f64 = match rec.front_face() {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let unit_direction: Vec3 = Vec3::unit_vector(&ray_in.direction());
        let refracted: Vec3 = Vec3::refract(unit_direction, rec.normal(), refraction_index);

        let scattered = Ray::new(rec.position(), refracted);

        Some((scattered, attenuation))
    }
}
