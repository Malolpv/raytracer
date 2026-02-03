use std::{
    fs::{File, OpenOptions},
    io::BufWriter,
    path::PathBuf,
};

use clap::Parser;

use crate::config::SceneLoader;

mod camera;
mod color;
mod components;
mod config;
mod materials;
mod ray;
mod scene;
mod utils;
mod vec3;
mod world;

use std::time::Instant;

fn main() {
    let args = config::Args::parse();

    let scene_conf_path: PathBuf = args.world;

    eprintln!(
        "Trying to load scene config from : {}",
        scene_conf_path.display()
    );

    match SceneLoader::from_json::<PathBuf>(scene_conf_path) {
        Ok(mut scene) => {
            // Time benchmark
            let start = Instant::now();

            // Setup the output writer and start rendering
            scene.render(&mut setup_buf());

            let elapsed = start.elapsed();

            eprintln!("Render took : {:?}", elapsed);
        }
        Err(e) => {
            eprintln!("An error occured while loading scene : {}", e)
        }
    }
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

/// Setup an output buffer
fn setup_buf() -> BufWriter<File> {
    // Create a 1MO buf writer
    BufWriter::with_capacity(1024 * 1024, setup())
}
