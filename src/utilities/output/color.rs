#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn can_get_default() {
        let color = Color::default();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn can_create() {
        let color = Color::new(1, 5, 50);
        assert_eq!(color.r, 1);
        assert_eq!(color.g, 5);
        assert_eq!(color.b, 50);
    }

    #[test]
    fn can_get_string() {
        let color = Color::new(1, 205, 50);
        assert_eq!(color.to_string(), String::from("1 205 50"));
    }
}
