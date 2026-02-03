use rayon::prelude::*;
use serde::Deserialize;

use crate::components::Hitable;
use crate::utils::random_f64;
use crate::world::World;
use crate::{
    color::{Color, WriteableColor},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    aspect_ratio: f64,
    image_width: usize,
    focal_length: f64,
    max_depth: u8,
    samples_per_pixel: usize,
}

impl CameraConfig {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        focal_length: f64,
        samples_per_pixel: usize,
        max_depth: u8,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            focal_length,
            max_depth,
            samples_per_pixel,
        }
    }
}

pub struct Camera {
    /// Ratio of image width over height
    aspect_ratio: f64,
    /// Rendered image width in pixel count
    image_width: usize,
    /// Rendered image height
    image_height: usize,

    /// Count of random samples for each pixel
    samples_per_pixel: usize,
    /// Color scale factor for a sum of pixel samples
    pixel_sample_scale: f64,
    /// Offset to pixel to the right
    pixel_delta_h: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Camera center
    center: Point3,
    /// Maximum number of ray bounces (smaller the depth -> darker the image)
    max_depth: u8,
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
            - Vec3::new(0_f64, 0_f64, config.focal_length)
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
            pixel_sample_scale: 1.0 / config.samples_per_pixel as f64,
            samples_per_pixel: config.samples_per_pixel,
            max_depth: config.max_depth,
        }
    }

    // keeping this function for benchmark purposes
    #[allow(dead_code)]
    pub fn render(&self, mut out: impl std::io::Write, world: &World) {
        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();
        for j in 0..self.image_height {
            Self::log_progress(&j, &self.image_height);
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::new(0_f64, 0_f64, 0_f64);

                for _sample in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, self.max_depth, world);
                }

                Color::write(&mut out, &(pixel_color * self.pixel_sample_scale));
            }
        }
    }

    // Keeping this function for benchmarks purpose
    #[allow(dead_code)]
    pub fn parallel_render(&self, mut out: impl std::io::Write, world: &World) {
        // Computing each pixels colors in parrallel execution
        let image_data: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                (0..self.image_width)
                    .map(|i| {
                        // Initialize a blank pixel
                        let mut pixel_color: Color = Color::white();

                        // Adding each sample color value into pixel
                        for _sample in 0..self.samples_per_pixel {
                            let ray: Ray = self.get_ray(i, j);
                            pixel_color += Self::ray_color(&ray, self.max_depth, world);
                        }

                        // Compute average color with the number of sample analyzed
                        pixel_color * self.pixel_sample_scale
                    })
                    .collect()
            })
            .collect();

        // Writing to ouput file in a single thread because File IO operation is slower when multithreaded
        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();
        for row in image_data {
            for pixel in row {
                Color::write(&mut out, &pixel);
            }
        }
    }

    /// Compute each pixels colors in parrallel
    /// Then write to given output
    pub fn optimized_parallel_render(&self, mut out: impl std::io::Write, world: &World) {
        // flatten the list to benefits from CPU prefetching
        let pixels: Vec<WriteableColor> = (0..self.image_height * self.image_width)
            .into_par_iter()
            .map(|index| {
                let i = index % self.image_width;
                let j = index / self.image_width;

                let mut pixel_color = Color::white();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, self.max_depth, world);
                }
                WriteableColor::from(&(pixel_color * self.pixel_sample_scale))
            })
            .collect();

        eprintln!("Finalized rendering computation. Writing to output...");

        // Writing to ouput file in a single thread because File IO operation is slower when multithreaded
        // PPM file header
        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();

        for pixel in pixels {
            // Writing to output, panic if something wrong happens
            writeln!(&mut out, "{} {} {}", pixel.r(), pixel.g(), pixel.b()).unwrap();
        }
    }

    fn ray_color(ray: &Ray, depth: u8, world: &World) -> Color {
        // If ray bounce limit is exceeded, no more light is gathered.
        if depth == 0 {
            return Color::white();
        }

        if let Some(hit) = world.hit(ray, 0.001..=f64::MAX) {
            if let Some((scattered, attenuation)) = hit.material().scatter(ray, &hit) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::white();

            // Randomly generate a vector according to the Lambertian Distribution
            // let direction: Vec3 = hit.normal() + Vec3::random_on_hemisphere(&hit.normal());

            // here the float number represent the gamut applied to the ray color
            // lower -> darker, higher -> clearer
            // return Self::ray_color(&Ray::new(hit.position(), direction), depth - 1, world) * 0.5;
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

    /// Construct a camera ray originating from the origin and directed at randomly sampled
    /// point around the pixel location i, j.
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset: Vec3 = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_h * (offset.x() + i as f64))
            + (self.pixel_delta_v * (j as f64 + offset.y()));

        let ray_origin: Point3 = self.center;
        let ray_direction: Vec3 = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0_f64)
    }
}
