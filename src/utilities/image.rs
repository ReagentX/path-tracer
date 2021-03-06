use std::{
    f64::consts::PI,
    fs::File,
    io::{BufWriter, Write},
    iter::repeat,
    path::Path,
    time::Instant,
};

use format_num::format_num;
use serde::{Deserialize, Serialize};

use crate::utilities::color::Color;

pub enum Orientation {
    Landscape,
    Portrait,
}

#[derive(Deserialize, Serialize)]
pub struct Image {
    pub width: u64,
    pub height: u64,
    #[serde(skip_serializing, skip_deserializing)]
    pub buffer: Vec<Color>,
}

impl Image {
    /// Fill the image buffer with the default color
    pub fn generate_buffer(width: u64, height: u64) -> Vec<Color> {
        let mut buffer: Vec<Color> = vec![Color::default(); (width * height) as usize];
        buffer.fill_with(Color::default);
        buffer
    }

    /// Written by Steven Sardegna
    fn get_index(&self, col: u64, row: u64) -> usize {
        ((self.height - 1 - row) * self.width + col) as usize
    }

    /// Get the color at a specific point in the image
    pub fn color_at(&mut self, col: u64, row: u64) -> &mut Color {
        let index = self.get_index(col, row);
        &mut self.buffer[index]
    }

    /// Create a canvas of the specified dimensions
    pub fn from_dimensions(width: u64, height: u64) -> Self {
        Image {
            width,
            height,
            buffer: Self::generate_buffer(width, height),
        }
    }

    /// Create a canvas given a height and the desired aspect ratio
    pub fn from_ratio(height: u64, aspect_ratio: f64) -> Self {
        let width = (height as f64 * aspect_ratio) as u64;
        Image {
            width,
            height,
            buffer: Self::generate_buffer(width, height),
        }
    }

    /// 16:9 widescreen canvas of the specified height
    pub fn widescreen(height: u64, orientation: Orientation) -> Self {
        match orientation {
            Orientation::Landscape => Image::from_ratio(height, 16. / 9.),
            Orientation::Portrait => Image::from_ratio(height, 9. / 16.),
        }
    }

    /// 1:1 square canvas of the specified height
    pub fn square(height: u64) -> Self {
        Image::from_ratio(height, 1.)
    }

    /// 4K image size
    pub fn uhd(orientation: Orientation) -> Self {
        match orientation {
            Orientation::Landscape => Image::from_dimensions(3840, 2160),
            Orientation::Portrait => Image::from_dimensions(2160, 3840),
        }
    }

    /// 2K image size
    pub fn qhd(orientation: Orientation) -> Self {
        match orientation {
            Orientation::Landscape => Image::from_dimensions(2560, 1440),
            Orientation::Portrait => Image::from_dimensions(1440, 2560),
        }
    }

    /// HD image size
    pub fn hd(orientation: Orientation) -> Self {
        match orientation {
            Orientation::Landscape => Image::from_dimensions(1920, 1080),
            Orientation::Portrait => Image::from_dimensions(1080, 1920),
        }
    }

    /// iPhone 13 Pro Max image size
    ///
    /// Scale reduces the size by that amount. A value of `scale = 4` renders at 1/4 scale.
    pub fn mobile(scale: u64, orientation: Orientation) -> Self {
        match orientation {
            Orientation::Landscape => Image::from_dimensions(2778 / scale, 1284 / scale),
            Orientation::Portrait => Image::from_dimensions(1284 / scale, 2778 / scale),
        }
    }

    /// Gets the number of pixels in the iamge
    pub fn pixels(&self) -> u64 {
        self.width * self.height
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    /// Returns an iterator that yields coordinate pairs, starting from
    /// (max_y, min_x), i.e. top left to bottom right, in the format of (row, col)
    pub fn walk(image: Self) -> impl Iterator<Item = (u64, u64)> {
        (0..image.height)
            .rev()
            .flat_map(move |row| repeat(row).zip(0..image.width))
    }

    /// Write the image buffer to a `.pmm` file
    pub fn save(&self, filepath: &str, filename: &str, gamma: f64) {
        // Generate filepath
        let path = Path::new(filepath).join(format!("{filename}.ppm"));

        // Create file
        let file = File::create(&path).unwrap();
        let mut buf_file = BufWriter::new(file);

        // Add ppm metadata
        writeln!(&mut buf_file, "P3").unwrap();
        writeln!(&mut buf_file, "{} {}", self.width, self.height).unwrap();
        writeln!(&mut buf_file, "255").unwrap();

        // Write ppm colors
        println!("Writing file...");
        let now = Instant::now();
        self.buffer.iter().for_each(|color| {
            buf_file
                .write_all(color.as_string(gamma).as_bytes())
                .unwrap();
        });
        buf_file.flush().unwrap();

        // Print metrics
        let elapsed = now.elapsed().as_millis();
        println!("Wrote data to {:0}", path.as_os_str().to_str().unwrap());
        if elapsed >= 1 {
            println!(
                "Wrote file in {:.2}s ({} pixels per milisecond)",
                elapsed as f64 / 1000.,
                format_num!(",d", self.buffer.len() as f64 / elapsed as f64)
            );
        }
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
    use crate::utilities::{color::Color, image::Image};

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
        assert_eq!(
            image.buffer,
            vec![
                Color::default(),
                Color::default(),
                Color::default(),
                Color::default(),
                Color::default(),
                Color::default(),
                Color::default(),
                Color::default(),
                Color::default(),
            ]
        );
    }

    #[test]
    fn can_walk() {
        let width = 3;
        let height = 3;
        let image = Image::from_dimensions(width, height);
        let mut walking_path = Image::walk(image);

        for i in (0..height).rev() {
            for j in 0..height {
                assert_eq!((i, j), walking_path.next().unwrap());
                println!("{:?}", (i, j));
            }
        }
    }

    #[test]
    fn can_get_valid_index_origin() {
        let image = Image::from_dimensions(3, 3);
        assert_eq!(image.get_index(0, 0), 6);
    }

    #[test]
    fn can_get_valid_color_origin() {
        let mut image = Image::from_dimensions(3, 3);
        assert_eq!(*image.color_at(0, 0), Color::default());
    }

    #[test]
    fn can_get_valid_index_middle() {
        let image = Image::from_dimensions(3, 3);
        assert_eq!(image.get_index(1, 1), 4)
    }

    #[test]
    fn can_get_valid_index_bottom_right() {
        let image = Image::from_dimensions(3, 3);
        assert_eq!(image.get_index(2, 0), 8)
    }

    #[test]
    fn can_get_valid_index_top_right() {
        let image = Image::from_dimensions(5, 3);
        assert_eq!(image.get_index(4, 2), 4)
    }
}
