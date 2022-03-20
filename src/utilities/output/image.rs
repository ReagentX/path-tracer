use std::{fmt::format, fs::File, io::Write, iter::repeat, path::Path};

use crate::utilities::output::color::Color;

pub struct Image {
    width: u64,
    height: u64,
    buffer: Vec<Color>,
}

impl Image {
    fn generate_buffer(width: u64, height: u64) -> Vec<Color> {
        let mut buffer: Vec<Color> = vec![Color::default(); (width * height) as usize];
        buffer.fill_with(Color::default);
        buffer
    }

    pub fn from_dimensions(width: u64, height: u64) -> Self {
        Image {
            width,
            height,
            buffer: Self::generate_buffer(width, height),
        }
    }

    pub fn from_ratio(height: u64, aspect_ratio: f64) -> Self {
        let width = (height as f64 * aspect_ratio) as u64;
        Image {
            width,
            height,
            buffer: Self::generate_buffer(width, height),
        }
    }

    pub fn walk(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        (0..self.height)
            .rev()
            .flat_map(|row| repeat(row).zip(0..self.width))
    }

    pub fn save(&self, filepath: &str, filename: &str) {
        // Generate filepath
        let path = Path::new(filepath).join(format!("{filename}.ppm"));

        // Create file
        let mut file = File::create(&path).unwrap();

        // Add ppm metadata
        writeln!(&mut file, "P3").unwrap();
        writeln!(&mut file, "{} {}", self.width, self.height).unwrap();
        writeln!(&mut file, "255").unwrap();

        // Write ppm colors
        println!("Before for loop");
        self.buffer.iter().for_each(|color| {
            println!("{}", color.to_string());
            writeln!(&mut file, "{}", color.to_string()).unwrap();
        });
        println!("Wrote data to {}", path.as_os_str().to_str().unwrap());
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            buffer: Self::generate_buffer(10, 10),
        }
    }
}

#[cfg(test)]
mod image_tests {
    use crate::utilities::output::{color::Color, image::Image};

    #[test]
    fn can_get_default() {
        let image = Image::default();
        assert_eq!(image.width, 10);
        assert_eq!(image.height, 10);
    }

    #[test]
    fn can_create_dimensions() {
        let image = Image::from_dimensions(25, 40);
        assert_eq!(image.width, 25);
        assert_eq!(image.height, 40);
    }

    #[test]
    fn can_create_ratio() {
        let image = Image::from_ratio(25, 1.77);
        assert_eq!(image.height, 25);
        assert_eq!(image.width, 44);
    }

    #[test]
    fn can_generate_buffer() {
        let image = Image::from_dimensions(3, 3);
        assert_eq!(image.buffer, vec![
            Color::default(),
            Color::default(),
            Color::default(),
            Color::default(),
            Color::default(),
            Color::default(),
            Color::default(),
            Color::default(),
            Color::default(),
        ]);
    }

    #[test]
    fn can_walk() {
        let width = 3;
        let height = 3;
        let image = Image::from_dimensions(width, height);
        let mut walking_path = image.walk();

        for i in (0..height).rev() {
            for j in 0..height {
                assert_eq!((i, j), walking_path.next().unwrap());
                println!("{:?}", (i, j));
            }
        }
    }
}
