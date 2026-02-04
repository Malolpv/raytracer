use std::{
    fs::{File, OpenOptions},
    io::BufWriter,
    path::PathBuf,
    sync::Arc,
};

use clap::Parser;

use crate::{
    camera::{Camera, CameraConfig},
    color::Color,
    components::Sphere,
    config::SceneLoader,
    materials::{dielectrics::Dielectrics, lambertian::Lambertian, metal::Metal},
    scene::Scene,
    utils::{random_f64, random_f64_in},
    vec3::{Point3, Vec3},
    world::World,
};

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
    // let args = config::Args::parse();

    // let scene_conf_path: PathBuf = args.world;

    // eprintln!(
    //     "Trying to load scene config from : {}",
    //     scene_conf_path.display()
    // );

    // match SceneLoader::from_json::<PathBuf>(scene_conf_path) {
    //     Ok(mut scene) => {
    //         // Time benchmark
    //         let start = Instant::now();

    //         // Setup the output writer and start rendering
    //         scene.render(&mut setup_buf());

    //         let elapsed = start.elapsed();

    //         eprintln!("Render took : {:?}", elapsed);
    //     }
    //     Err(e) => {
    //         eprintln!("An error occured while loading scene : {}", e)
    //     }
    // }

    let mut world: World = World::new();

    // Ground Sphere
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground_sphere = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    );
    world.add(components::Component::Sphere(ground_sphere));

    // Small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_f64();

            let center: Point3 = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // Diffuse material
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::new(albedo);
                    world.add(components::Component::Sphere(Sphere::new(
                        center,
                        0.2,
                        Arc::new(material),
                    )));
                } else if choose_material < 0.95 {
                    // Metal material
                    let albedo = Color::random_from_range(0.5..1.0);
                    let fuzz = random_f64_in(0.0..0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.add(components::Component::Sphere(Sphere::new(
                        center,
                        0.2,
                        Arc::new(material),
                    )));
                } else {
                    // Glass material
                    let material = Dielectrics::new(1.5);
                    world.add(components::Component::Sphere(Sphere::new(
                        center,
                        0.2,
                        Arc::new(material),
                    )));
                }
            }
        }
    }

    // Big glass sphere
    let material = Dielectrics::new(1.5);
    world.add(components::Component::Sphere(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(material),
    )));

    // Big Diffuse sphere
    let material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(components::Component::Sphere(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(material),
    )));

    // Big Metal sphere
    let material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(components::Component::Sphere(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material),
    )));

    let camera = Camera::new(CameraConfig::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        10.0,
        0.6,
    ));

    let mut scene = Scene::new(camera, world);
    // Time benchmark
    let start = Instant::now();
    scene.render(&mut setup_buf());
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

/// Setup an output buffer
fn setup_buf() -> BufWriter<File> {
    // Create a 1MO buf writer
    BufWriter::with_capacity(1024 * 1024, setup())
}
