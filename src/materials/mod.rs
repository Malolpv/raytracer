use serde::Deserialize;

use crate::{color::Color, components::HitRecord, ray::Ray};

pub mod lambertian;
pub mod metal;

pub trait Material: Send + Sync {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

#[derive(Deserialize)]
pub enum MaterialType {
    // Lambertian(Lambertian),
    // Metal(Metal),
}
