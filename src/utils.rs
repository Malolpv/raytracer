use std::ops::Range;

use rand::Rng;

/// Generate a random `f64` between 0 and 1, 1 exluded
pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0_f64..1_f64)
}

pub fn random_f64_in(range: Range<f64>) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_random_between_0_1() {
        // Arrange
        let range = 0_f64..1_f64;
        // Act
        let number: f64 = random_f64();

        // Assert
        assert!(range.contains(&number));
    }

    #[test]
    fn test_random_range() {
        // Arrange
        let range = 5_f64..6_f64;

        // Act
        let number: f64 = random_f64_in(range.clone());

        // Assert
        assert!(range.contains(&number));
    }
}
