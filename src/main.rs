use std::{
    fs::{File, OpenOptions},
    io::BufWriter,
};

use crate::{
    camera::{Camera, CameraConfig},
    scene::Scene,
    world::MockWorld,
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
    let available_threads = std::thread::available_parallelism().unwrap();
    eprintln!("{}", available_threads);
    let config: CameraConfig = CameraConfig::new(16.0 / 9.0, 3840, 1.0, 100, 50);

    eprintln!("Camera configuration: {:?}", config);

    let camera: Camera = Camera::new(config);

    // let mut out = std::io::stdout();
    let mut out = setup_buf();

    let mut scene = Scene::new(
        camera,
        MockWorld::get_mock_world(MockWorld::BigAndSmallSpheres),
    );

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

fn setup_buf() -> BufWriter<File> {
    // Create a 1MO buf writer
    BufWriter::with_capacity(1024 * 1024, setup())
}
