use crate::{camera::Camera, world::World};

pub struct Scene {
    world: World,
    camera: Camera,
}

impl Scene {
    /// Create a new scene object with the given camera & world
    ///
    /// Call `render` to start rendering the image
    pub fn new(camera: Camera, world: World) -> Self {
        Self { world, camera }
    }

    pub fn render(&mut self, out: impl std::io::Write) {
        eprintln!("Starting Rendering");
        self.camera.parallel_render(out, &self.world);
        eprintln!("Rendering complete");
    }
}
