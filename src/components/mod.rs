pub mod sphere;

use std::ops::{Range, RangeInclusive};

use serde::Deserialize;
pub use sphere::Sphere;

use crate::{ray::Ray, vec3::Point3, vec3::Vec3};

pub struct HitRecord {
    position: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(position: Point3, normal: Vec3, t: f64, ray: &Ray) -> Self {
        let mut tmp = Self {
            position,
            normal,
            t,
            front_face: false,
        };

        tmp.set_face_normal(ray, &normal);
        tmp
    }

    pub fn position(&self) -> Point3 {
        self.position
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    //TODO Maybe refactor this to use self.normal instead of outward_normal
    /// Sets the hit record normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction(), outward_normal) < 0_f64;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<HitRecord>;
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Component {
    Sphere(Sphere),
    // Triangle(Triangle),
    // Rectangle(Rectangle),
    // ...
}

impl Hitable for Component {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, t_range),
        }
    }
}
