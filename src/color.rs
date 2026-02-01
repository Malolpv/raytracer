use std::{io::Write, ops};

use serde::Deserialize;

use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn white() -> Self {
        Self {
            r: 0_f64,
            g: 0_f64,
            b: 0_f64,
        }
    }

    /// Apply a linear to gamma transform for gamma 2
    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0_f64 {
            return linear_component.sqrt();
        }
        0_f64
    }

    pub fn as_u8_array(pixel_color: &Color) -> [u8; 3] {
        // Translate the [0,1] component values to the byte range [0,255].
        const INTERVAL: ops::Range<f64> = 0_f64..0.999;

        // Applying gamma correction in order to have a consistent ramp from darkness to lightness
        let r = Self::linear_to_gamma(pixel_color.r);
        let g = Self::linear_to_gamma(pixel_color.g);
        let b = Self::linear_to_gamma(pixel_color.b);

        // convert to 8 bits values
        let r_byte: u8 = (256_f64 * r.clamp(INTERVAL.start, INTERVAL.end)) as u8;
        let g_byte: u8 = (256_f64 * g.clamp(INTERVAL.start, INTERVAL.end)) as u8;
        let b_byte: u8 = (256_f64 * b.clamp(INTERVAL.start, INTERVAL.end)) as u8;

        [r_byte, g_byte, b_byte]
    }

    pub fn write(out: &mut impl Write, pixel_color: &Color) {
        // Translate the [0,1] component values to the byte range [0,255].
        const INTERVAL: ops::Range<f64> = 0_f64..0.999;

        // Applying gamma correction in order to have a consistent ramp from darkness to lightness
        let r = Self::linear_to_gamma(pixel_color.r);
        let g = Self::linear_to_gamma(pixel_color.g);
        let b = Self::linear_to_gamma(pixel_color.b);

        // convert to 8 bits values
        let r_byte: u8 = (256_f64 * r.clamp(INTERVAL.start, INTERVAL.end)) as u8;
        let g_byte: u8 = (256_f64 * g.clamp(INTERVAL.start, INTERVAL.end)) as u8;
        let b_byte: u8 = (256_f64 * b.clamp(INTERVAL.start, INTERVAL.end)) as u8;

        // Just panic if anything breaks
        writeln!(out, "{r_byte} {g_byte} {b_byte}").unwrap();
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl ops::Add<Vec3> for Color {
    type Output = Color;

    fn add(self, other: Vec3) -> Self {
        Color {
            r: self.r + other.x(),
            g: self.g + other.y(),
            b: self.b + other.z(),
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

pub struct WriteableColor {
    r: u8,
    g: u8,
    b: u8,
}

impl WriteableColor {
    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, b, g }
    }

    pub fn from(pixel_color: &Color) -> Self {
        // Translate the [0,1] component values to the byte range [0,255].
        const INTERVAL: ops::Range<f64> = 0_f64..0.999;

        // Applying gamma correction in order to have a consistent ramp from darkness to lightness
        let r = Color::linear_to_gamma(pixel_color.r);
        let g = Color::linear_to_gamma(pixel_color.g);
        let b = Color::linear_to_gamma(pixel_color.b);

        // convert to 8 bits values
        Self::new(
            (256_f64 * r.clamp(INTERVAL.start, INTERVAL.end)) as u8,
            (256_f64 * g.clamp(INTERVAL.start, INTERVAL.end)) as u8,
            (256_f64 * b.clamp(INTERVAL.start, INTERVAL.end)) as u8,
        )
    }
}
