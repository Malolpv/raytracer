use crate::components::{Component, HitRecord, Hitable};

pub struct World {
    objects: Vec<Component>, //Vec<Box<dyn Hitable>>,
}

impl Hitable for World {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_range: std::ops::RangeInclusive<f64>,
    ) -> Option<crate::components::HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_object: f64 = *t_range.end();

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, *t_range.start()..=closest_object) {
                closest_object = hit.t();
                rec = Some(hit);
            }
        }

        rec
    }
}

impl World {
    /// Return an empty world
    pub fn new() -> Self {
        let vec: Vec<Component> = Vec::new();
        Self { objects: vec }
    }

    pub fn add(&mut self, object: Component) {
        self.objects.push(object);
    }
}
