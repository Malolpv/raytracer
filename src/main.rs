use std::fs::{File, OpenOptions};

use crate::{
    camera::{Camera, CameraConfig},
    scene::Scene,
    world::World,
};

mod camera;
mod color;
mod components;
mod ray;
mod scene;
mod utils;
mod vec3;
mod world;

use std::time::Instant;

fn main() {
    let config: CameraConfig = CameraConfig::new(16.0 / 9.0, 400, 1.0, 100, 50);

    eprintln!("Camera configuration: {:?}", config);

    let camera: Camera = Camera::new(config);

    // let mut out = std::io::stdout();
    let mut out = setup();

    let mut scene = Scene::new(camera, World::get_mock_world());

    // Time benchmark
    let start = Instant::now();

    scene.render(&mut out);

    let elapsed = start.elapsed();

    eprintln!("Render took : {:?}", elapsed);
}

fn setup() -> File {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("./img.ppm")
        .unwrap();

    file
}
