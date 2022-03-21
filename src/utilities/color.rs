#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha component
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
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
        assert_eq!(color.a, 255);
    }

    #[test]
    fn can_create() {
        let color = Color::new(1, 5, 50, 100);
        assert_eq!(color.r, 1);
        assert_eq!(color.g, 5);
        assert_eq!(color.b, 50);
        assert_eq!(color.a, 100);
    }

    #[test]
    fn can_get_string() {
        let color = Color::new(1, 205, 50, 255);
        assert_eq!(color.to_string(), String::from("1 205 50\n"));
    }
}
