use crate::{color::Color, components::HitRecord, ray::Ray};

pub mod dielectrics;
pub mod lambertian;
pub mod metal;

pub trait Material: Send + Sync {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}
