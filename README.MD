# Rust Ray Tracing Engine

A 3D ray tracer implemented in Rust. This project is strictly based on the mathematical foundations and architecture described in the **"Ray Tracing in One Weekend"** book series by Peter Shirley, Trevor David Black, and Steve Hollasch.

For further information on the original series, please visit [raytracing.github.io](https://raytracing.github.io/).

## Overview

The engine simulates the physical behavior of light by casting rays from a virtual camera into a scene composed of geometric primitives. It calculates intersections, surface normals, and color gradients to produce 2D images in the PPM (Portable Pixmap) format.

## Goal

My goals with this project are:
* **Rust Proficiency**: To develop advanced skills in the Rust programming language, focusing on memory safety, concurrency, and performance optimization.
* **Software Architecture**: Learning how to design a modular engine where components (camera, geometry, materials) are decoupled and easy to test.
* **Extensible Design**: Thinking ahead about how to implement future features (like new geometries, complex materials, or textures) without refactoring the entire core pipeline.
* **Graphics Computing Exploration**: To discover the fundamental principles of 3D rendering and computer graphics by building this RayTracer from scratch.

## Core Architecture

The project follows a modular design to ensure clarity and performance:

* **Linear Algebra**: A custom implementation of 3D vectors (`Vec3`) and points (`Point3`) with operator overloading for geometric calculations.
* **Ray Abstraction**: Definition of rays as vectors of light.
* **Camera System**: Handles the mapping of the image grid to the 3D viewport, focal length, and ray generation.
* **Geometry Management**: A `World` container that manages scene objects and implements efficient hit-detection loops.
* **Primitive Intersections**: Optimized ray-sphere intersection logic using the quadratic formula and discriminant analysis.
* **Parallelism**: Support for multi-threaded rendering across scanlines using the `Rayon` crate.

## Technical Details

* **PPM Output**: Images are generated with a standard P3 header and 8-bit color depth per channel.
* **Aspect Ratio Management**: Dynamic calculation of viewport dimensions based on the actual pixel grid to prevent geometric distortion.
* **Memory Safety**: Utilizes Rust's ownership model to safely share scene data across multiple threads during the rendering phase.

## Project Structure

```text
src/
├── components/
│   ├── mod.rs      # Hitable trait and HitRecord definitions
│   └── sphere.rs   # Sphere intersection logic
├── camera.rs       # Viewport and ray generation
├── color.rs        # Color utilities and PPM formatting
├── main.rs         # Application entry point and configuration
├── ray.rs          # Ray structure
├── scene.rs        # High-level scene orchestration
├── vec3.rs         # Vector mathematics
└── world.rs        # Object collection and collision loops
```

## Getting Started

### Prerequisites 
* **Rust Compiler:** Edition 2021 or later
* **Cargo:** Rust's package manager

### Execution

1. **Clone the repository** 

```bash
git clone https://github.com/Malolpv/raytracer.git
cd raytracer
```

2. **Build and Run in Release mode**

Compiling with the --release flag is highly recommended for better performances.
```bash
cargo run --release
```

3. **Visualize output**

By default this renderer produces an ``img.ppm`` file in the project directory. You can configure this by editing the ``out`` variable in ``main.rs`` (``out`` must implement ``std::io::Write``).

Then you can use anytool that supports the ppm format like [Gimp](https://www.gimp.org/downloads/) to actually view the image.

## Acknowledgments

Special Thanks to **Peter Shirley**, **Trevor David Black**, **Steve Hollasch** and the **contributors** of the **Ray Tracing in One Weekend series** for providing the foundational educational resources.s
