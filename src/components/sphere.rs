use std::{ops::RangeInclusive, sync::Arc};

use crate::{
    components::Hitable,
    materials::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    radius: f64,
    position: Point3,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(position: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            radius,
            position,
            material,
        }
    }

    pub fn radius(&self) -> &f64 {
        &self.radius
    }

    pub fn position(&self) -> &Point3 {
        &self.position
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<super::HitRecord> {
        let oc: Vec3 = self.position - ray.origin();

        let a: f64 = ray.direction().length_squared();
        let h: f64 = Vec3::dot(&ray.direction(), &oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant: f64 = h.powi(2) - a * c;
        if discriminant < 0_f64 {
            return None;
        }

        let disc_sqrt = discriminant.sqrt();

        // Limit search for intersection to the range t_min tmax
        let mut root = (h - disc_sqrt) / a;
        if !(t_range).contains(&root) {
            root = (h + disc_sqrt) / a;
            if !(t_range).contains(&root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.position) / self.radius;

        Some(super::HitRecord::new(
            p,
            normal,
            t,
            ray,
            self.material.clone(),
        ))
    }
}
