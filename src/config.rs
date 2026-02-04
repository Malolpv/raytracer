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
    materials::{dielectrics::Dielectrics, lambertian::Lambertian, metal::Metal, Material},
    scene::Scene,
    vec3::{Point3, Vec3},
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
    max_depth: u8,
    samples_per_pixel: usize,
    vfov: f64,
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
}

impl CameraJsonConfig {
    fn into_camera(self) -> Camera {
        Camera::new(CameraConfig::new(
            self.aspect_ratio.to_f64(),
            self.image_width,
            self.samples_per_pixel,
            self.max_depth,
            self.vfov,
            self.look_from,
            self.look_at,
            self.vup,
            self.focus_dist,
            self.defocus_angle,
        ))
    }
}

// This struct allows to specify the refraction index as a fraction
#[derive(serde::Deserialize)]
/// Wrapper to represent a refraction index
///
/// To render glass in the air :
///
/// material_ri => refraction index of glass (1.5)
///
/// enclosing_ri => refraction index of air (1.0)
struct RefractionIndex {
    material_ri: f64,
    enclosing_ri: f64,
}

impl RefractionIndex {
    fn to_f64(&self) -> f64 {
        self.material_ri / self.enclosing_ri
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum MaterialConfig {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectrics { refraction_index: RefractionIndex },
}

impl MaterialConfig {
    /// Converts config to an actual material
    pub fn build(self) -> Arc<dyn Material> {
        match self {
            MaterialConfig::Lambertian { albedo } => Arc::new(Lambertian::new(albedo)),
            MaterialConfig::Metal { albedo, fuzz } => Arc::new(Metal::new(albedo, fuzz)),
            Self::Dielectrics { refraction_index } => {
                Arc::new(Dielectrics::new(refraction_index.to_f64()))
            }
        }
    }
}

#[derive(Deserialize)]
pub struct SphereConfig {
    position: Point3,
    radius: f64,
    material: MaterialConfig,
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
