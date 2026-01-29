use crate::{
    color::Color,
    components::{Component, HitRecord, Hitable, Sphere},
    vec3::Point3,
};

pub struct World {
    objects: Vec<Component>, //Vec<Box<dyn Hitable>>,
}

impl Hitable for World {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_range: std::ops::Range<f64>,
    ) -> Option<crate::components::HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_object: f64 = t_range.end;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_range.start..closest_object) {
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

    pub fn get_mock_world() -> World {
        eprintln!("Generating a mock world");
        let mut world = World::new();

        // Medium left sphere
        world.add(Component::Sphere(Sphere::new(
            Point3::new(-0.5, 0.0, -1.0),
            0.25,
            Color::new(1.0, 1.0, 0.0),
        )));

        // Big right sphere
        world.add(Component::Sphere(Sphere::new(
            Point3::new(0.5, 0.0, -1.0),
            0.5,
            Color::new(1.0, 1.0, 0.0),
        )));

        // Big ground sphere
        world.add(Component::Sphere(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Color::new(0.0, 0.0, 1.0),
        )));

        world
    }
}
