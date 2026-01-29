use std::{io::Write, ops};

use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn write(out: &mut impl Write, pixel_color: &Color) {
        // Translate the [0,1] component values to the byte range [0,255].
        const INTERVAL: ops::Range<f64> = 0_f64..0.999;
        let r_byte: usize = (256_f64 * pixel_color.r.clamp(INTERVAL.start, INTERVAL.end)) as usize;
        let g_byte: usize = (256_f64 * pixel_color.g.clamp(INTERVAL.start, INTERVAL.end)) as usize;
        let b_byte: usize = (256_f64 * pixel_color.b.clamp(INTERVAL.start, INTERVAL.end)) as usize;

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
