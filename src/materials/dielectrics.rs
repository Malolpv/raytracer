use crate::{color::Color, materials::Material, ray::Ray, vec3::Vec3};

pub struct Dielectrics {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over the refractive index of the enclosing media
    ///
    /// ex: to render a air bubble in water : refractive index of air / refractive index of water
    ///
    /// which equals to => (1.0/1.333)
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
        // Glass surface absorb nothing so attenuation is always 1
        let attenuation: Color = Color::new(1_f64, 1_f64, 1_f64);
        let refraction_index: f64 = match rec.front_face() {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let unit_direction: Vec3 = Vec3::unit_vector(&ray_in.direction());

        //  Snell's law. When a ray enters a medium of lower index of refraction at a sufficiently glancing angle, it can refract with an angle greater than 90°
        let cos_theta = Vec3::dot(&(unit_direction * -1f64), &rec.normal()).min(1_f64);
        let sin_theta = (1_f64 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_index * sin_theta > 1_f64;

        let direction = match cannot_refract {
            // Material cannot refract so it must reflect the ray
            true => Vec3::reflect(unit_direction, rec.normal()),
            false => Vec3::refract(unit_direction, rec.normal(), refraction_index),
        };

        let scattered = Ray::new(rec.position(), direction);

        Some((scattered, attenuation))
    }
}
