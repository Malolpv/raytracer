use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::{Parser, ValueHint};
use serde::Deserialize;

use crate::{
    camera::{Camera, CameraConfig},
    color::Color,
    components::{Component, Sphere},
    materials::{lambertian::Lambertian, metal::Metal, Material},
    scene::Scene,
    vec3::Point3,
    world::World,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the scene configuration, must be a valid json file
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub world: PathBuf,
}

// This struct allows to specify the aspect ratio as a fraction
#[derive(serde::Deserialize)]
struct AspectRatio {
    width: f64,
    height: f64,
}

impl AspectRatio {
    fn to_f64(&self) -> f64 {
        self.width / self.height
    }
}

#[derive(Deserialize)]
struct CameraJsonConfig {
    aspect_ratio: AspectRatio,
    image_width: usize,
    focal_length: f64,
    max_depth: u8,
    samples_per_pixel: usize,
}

impl CameraJsonConfig {
    fn into_camera(self) -> Camera {
        Camera::new(CameraConfig::new(
            self.aspect_ratio.to_f64(),
            self.image_width,
            self.focal_length,
            self.samples_per_pixel,
            self.max_depth,
        ))
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MaterialConfig {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

impl MaterialConfig {
    /// La méthode magique qui transforme la config en objet thread-safe
    pub fn build(self) -> Arc<dyn Material> {
        match self {
            MaterialConfig::Lambertian { albedo } => Arc::new(Lambertian::new(albedo)),
            MaterialConfig::Metal { albedo } => Arc::new(Metal::new(albedo)),
        }
    }
}

#[derive(Deserialize)]
pub struct SphereConfig {
    position: Point3,
    radius: f64,
    material: MaterialConfig, // On lit la config ici
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ComponentConfig {
    Sphere(SphereConfig),
}

#[derive(Deserialize)]
struct WorldConfig {
    objects: Vec<ComponentConfig>,
}

impl WorldConfig {
    fn into_world(self) -> World {
        let mut world = World::new();
        for object in self.objects {
            match object {
                ComponentConfig::Sphere(s) => {
                    let mat = s.material.build();
                    world.add(Component::Sphere(Sphere::new(s.position, s.radius, mat)));
                }
            }
        }
        world
    }
}

#[derive(Deserialize)]
pub struct SceneConfig {
    #[serde(rename = "camera")]
    camera_conf: CameraJsonConfig,
    #[serde(rename = "world")]
    world_conf: WorldConfig,
}

impl SceneConfig {
    fn into_scene(self) -> Scene {
        Scene::new(self.camera_conf.into_camera(), self.world_conf.into_world())
    }
}
pub struct SceneLoader;

impl SceneLoader {
    /// Load a scene from the given config file
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Scene, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);

        // Parsing scene config from the json file
        let conf: SceneConfig = serde_json::from_reader(reader)?;

        // Convert the parsed config into a scene
        let scene = conf.into_scene();

        Ok(scene)
    }
}
