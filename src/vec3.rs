use std::ops::{self, Range};

use crate::utils::{random_f64, random_f64_in};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(vec: &Self) -> Self {
        *vec / vec.length()
    }

    /// Instanciate a new vector with random values in range (0, 1]
    pub fn random() -> Self {
        Self::new(random_f64(), random_f64(), random_f64())
    }

    pub fn random_from_range(range: Range<f64>) -> Self {
        Self::new(
            random_f64_in(range.clone()),
            random_f64_in(range.clone()),
            random_f64_in(range),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random();
            let len_sq = p.length_squared();
            if 1e-160 < len_sq && len_sq <= 1_f64 {
                return p / len_sq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();

        if Self::dot(&on_unit_sphere, normal) > 0_f64 {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1_f64 / rhs)
    }
}

// TODO maybe this isnt good
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1_f64 / rhs
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

pub type Point3 = Vec3;

impl Vec3 {
    /// Calculates the dot product of two vectors.
    ///
    /// The dot product is a scalar value that represents the magnitude of
    /// one vector in the direction of another. It is calculated as:
    /// x1*x2 + y1*y2 + z1*z2.
    ///
    /// # Geometric Significance
    /// * If dot > 0: The angle between vectors is less than 90 degrees.
    /// * If dot = 0: The vectors are perpendicular (orthogonal).
    /// * If dot < 0: The vectors point in generally opposite directions.
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Calculates the cross product of two vectors.
    ///
    /// The cross product returns a new vector that is perpendicular (normal)
    /// to the plane containing the two input vectors.
    ///
    /// # Mathematical Note
    /// The direction of the resulting vector follows the "right-hand rule."
    /// If the input vectors are parallel, the result will be a zero vector.
    ///
    /// # Applications
    /// Commonly used to find the surface normal of a triangle or to
    /// construct an orthonormal basis for a camera system.
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_2_vec3() {
        // Arrange
        let vec1 = Vec3::new(3.0, 3.0, 3.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);

        // Act
        let result = vec1 + vec2;
        // Assert
        assert_eq!(result, Vec3::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn add_assign() {
        // Arrange
        let mut vec1 = Vec3::new(2.0, 2.0, 2.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);

        // Act
        vec1 += vec2;

        // Assert
        assert_eq!(vec1, Vec3::new(3.0, 3.0, 3.0))
    }

    #[test]
    fn test_mul_scalar() {
        // Arrange
        let v = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 2.0;

        // Act
        let result = v * scalar;
        // Assert
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_assign_scalar() {
        // Arrange
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 3.0;

        // Act
        v *= scalar;

        // Assert
        assert_eq!(v, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_sub() {
        // Arrange
        let v1 = Vec3::new(10.0, 5.0, 2.0);
        let v2 = Vec3::new(2.0, 1.0, 1.0);

        // Act
        let res = v1 - v2;

        // Assert
        assert_eq!(res, Vec3::new(8.0, 4.0, 1.0));
    }

    #[test]
    fn test_sub_assign() {
        // Arrange
        let mut v = Vec3::new(10.0, 10.0, 10.0);

        // Act
        v -= Vec3::new(1.0, 2.0, 3.0);

        // Assert
        assert_eq!(v, Vec3::new(9.0, 8.0, 7.0));
    }

    #[test]
    fn test_div() {
        // Arrange
        let v = Vec3::new(10.0, 20.0, 30.0);

        // Act
        let res = v / 2.0;

        // Assert
        assert_eq!(res, Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_div_assign() {
        // Arrange
        let mut v = Vec3::new(10.0, 20.0, 30.0);

        // Act
        v /= 2.0;

        // Assert
        assert_eq!(v, Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_negative() {
        // Arrange
        let v = Vec3::new(10.0, 20.0, 30.0);

        // Act
        let res = -v;

        // Assert
        assert_eq!(res, Vec3::new(-10.0, -20.0, -30.0))
    }
}
