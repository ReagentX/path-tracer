use std::ops::{Add, Mul};

use rand::distributions::{Distribution, Uniform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// Red component
    pub r: f64,
    /// Green component
    pub g: f64,
    /// Blue component
    pub b: f64,
    /// Alpha component
    pub a: u8,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Format the color as a ppm triplet, applying gamma correction
    pub fn as_string(&self, gamma: f64) -> String {
        if gamma > 0. {
            let ir = (256.0 * (self.r.powf(1.0 / gamma)).clamp(0.0, 1.0)) as u64;
            let ig = (256.0 * (self.g.powf(1.0 / gamma)).clamp(0.0, 1.0)) as u64;
            let ib = (256.0 * (self.b.powf(1.0 / gamma)).clamp(0.0, 1.0)) as u64;
            return format!("{} {} {}\n", ir, ig, ib);
        }
        return format!(
            "{} {} {}\n",
            (256.0 * self.r.clamp(0.0, 1.0)) as u64,
            (256.0 * self.g.clamp(0.0, 1.0)) as u64,
            (256.0 * self.b.clamp(0.0, 1.0)) as u64
        );
    }

    /// Generate a random color
    pub fn random() -> Color {
        let between = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        Color::new(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
            255,
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 255,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs, self.a)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b, rhs.a)
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b, self.a)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn can_get_default() {
        let color = Color::default();
        assert!((color.r - 0.).abs() < f64::EPSILON);
        assert!((color.g - 0.).abs() < f64::EPSILON);
        assert!((color.b - 0.).abs() < f64::EPSILON);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn can_create() {
        let color = Color::new(0.2, 0.6, 0.8, 100);
        assert!((color.r - 0.2).abs() < f64::EPSILON);
        assert!((color.g - 0.6).abs() < f64::EPSILON);
        assert!((color.b - 0.8).abs() < f64::EPSILON);
        assert_eq!(color.a, 100);
    }

    #[test]
    fn can_create_rgb() {
        let color = Color::rgb(0.2, 0.6, 0.8);
        assert!((color.r - 0.2).abs() < f64::EPSILON);
        assert!((color.g - 0.6).abs() < f64::EPSILON);
        assert!((color.b - 0.8).abs() < f64::EPSILON);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn can_get_string() {
        let color = Color::new(1., 0.8, 0.3, 255);
        assert_eq!(color.as_string(0.), String::from("256 204 76\n"));
    }

    #[test]
    fn can_get_string_out_of_bounds() {
        let color = Color::new(2., 0.8, -3.0, 255);
        assert_eq!(color.as_string(0.), String::from("256 204 0\n"));
    }

    #[test]
    fn can_get_string_gamma_brighter() {
        let color = Color::new(0.1, 0.8, 0.3, 255);
        assert_eq!(color.as_string(1.1), String::from("31 208 85\n"));
    }

    #[test]
    fn can_get_string_gamma_darker() {
        let color = Color::new(0.1, 0.8, 0.3, 255);
        assert_eq!(color.as_string(0.9), String::from("19 199 67\n"));
    }

    #[test]
    fn can_mul_float_color() {
        let mut color = Color::new(0.2, 0.6, 0.8, 100);
        color = 0.5 * color;
        assert!((color.r - 0.1).abs() < f64::EPSILON);
        assert!((color.g - 0.3).abs() < f64::EPSILON);
        assert!((color.b - 0.4).abs() < f64::EPSILON);
        assert_eq!(color.a, 100);
    }

    #[test]
    fn can_mul_color_float() {
        let mut color = Color::new(0.2, 0.6, 0.8, 100);
        color = color * 0.5;
        assert!((color.r - 0.1).abs() < f64::EPSILON);
        assert!((color.g - 0.3).abs() < f64::EPSILON);
        assert!((color.b - 0.4).abs() < f64::EPSILON);
        assert_eq!(color.a, 100);
    }

    #[test]
    fn can_mul_color_color() {
        let color_1 = Color::new(0.3, 1., 0.9, 255);
        let color_2 = Color::new(0.1, 0.2, 0.3, 255);
        let color_3 = color_1 * color_2;
        assert!((color_3.r - 0.03).abs() < f64::EPSILON);
        assert!((color_3.g - 0.2).abs() < f64::EPSILON);
        assert!((color_3.b - 0.27).abs() < f64::EPSILON);
        assert_eq!(color_3.a, 255);
    }

    #[test]
    fn can_add() {
        let color_1 = Color::new(0.2, 0.1, 0.9, 255);
        let color_2 = Color::new(0.1, 0.2, 0.3, 255);
        let color_3 = color_1 + color_2;
        assert!((color_3.r - 0.3).abs() < f64::EPSILON);
        assert!((color_3.g - 0.3).abs() < f64::EPSILON);
        assert!((color_3.b - 1.2).abs() < f64::EPSILON);
        assert_eq!(color_3.a, 255);
    }

    #[test]
    fn test_can_make_gray() {
        let color = 0.5 * Color::new(1.0, 1.0, 1.0, 255) + 0.5 * Color::new(0.5, 0.7, 1.0, 255);
        assert_eq!(color.r, 0.75);
        assert_eq!(color.g, 0.85);
        assert_eq!(color.b, 1.);
        assert_eq!(color.a, 255);
        assert_eq!(color.as_string(0.), String::from("192 217 256\n"));
    }
}
