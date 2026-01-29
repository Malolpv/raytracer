use crate::components::Hitable;

use crate::world::World;
use crate::{
    color::Color,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub struct CameraConfig {
    aspect_ratio: f64,
    image_width: usize,
    focal_lenght: f64,
}

impl CameraConfig {
    pub fn new(aspect_ratio: f64, image_width: usize, focal_lenght: f64) -> Self {
        Self {
            aspect_ratio,
            image_width,
            focal_lenght,
        }
    }
}

pub struct Camera {
    aspect_ratio: f64,
    image_width: usize,
    image_height: usize,

    pixel_delta_h: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    center: Point3,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        // Compute image height and ensure its >= 1
        let image_height: f64 = config.image_width as f64 / config.aspect_ratio;

        let image_height: usize = if image_height < 1f64 {
            1
        } else {
            image_height as usize
        };

        //reupdate aspect ratio after casting to usize
        let actual_aspect_ratio = config.image_width as f64 / image_height as f64;

        // Compute viewport dimensions
        let viewport_height: f64 = 2_f64;
        let viewport_width: f64 = viewport_height * actual_aspect_ratio;

        // Compute the vectors accross the horizontal and down vertical viewport edges
        let viewport_h: Vec3 = Vec3::new(viewport_width, 0_f64, 0_f64);
        let viewport_v: Vec3 = Vec3::new(0_f64, -viewport_height, 0_f64);

        // Compute the horizontal and vertical delta vectos from pixel to pixel
        let pixel_delta_h: Vec3 = viewport_h / config.image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

        let camera_center: Point3 = Point3::new(0_f64, 0_f64, 0_f64);

        let viewport_upper_left_pixel = camera_center
            - Vec3::new(0_f64, 0_f64, config.focal_lenght)
            - viewport_h / 2_f64
            - viewport_v / 2_f64;

        let pixel00_loc: Point3 = viewport_upper_left_pixel + (pixel_delta_h + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio: config.aspect_ratio,
            image_width: config.image_width,
            image_height,
            pixel_delta_h,
            pixel_delta_v,
            pixel00_loc,
            center: camera_center,
        }
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_h * i as f64) + (self.pixel_delta_v * j as f64);

        let ray_direction = pixel_center - self.center;

        Ray::new(self.center, ray_direction)
    }

    pub fn render(&self, mut out: impl std::io::Write, world: &World) {
        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();
        for j in 0..self.image_height {
            Self::log_progress(&j, &self.image_height);
            for i in 0..self.image_width {
                let ray = self.get_ray(i, j);

                Color::write(&mut out, &Self::ray_color(&ray, world));
            }
        }
    }

    fn ray_color(ray: &Ray, world: &World) -> Color {
        if let Some(hit) = world.hit(ray, 0_f64..f64::MAX) {
            return (Color::new(1_f64, 1_f64, 1_f64) + hit.normal()) * 0.5;
        }

        let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
        let a: f64 = 0.5 * (unit_direction.y() + 1.0);

        Color::new(1_f64, 1_f64, 1_f64) * (1_f64 - a) + Color::new(0.5, 0.7, 1_f64) * a
    }

    fn log_progress(progress: &usize, max: &usize) {
        let step = max / 10;

        if step > 0 && progress % step == 0 {
            let percentage = ((*progress as f64 / *max as f64) * 100.0).round() as usize;

            eprintln!("Rendered {}% of the image", percentage);
        }
    }
}
