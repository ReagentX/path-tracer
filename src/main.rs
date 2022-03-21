mod utilities;

use crate::utilities::{color::Color, image::Image};
use std::time::Instant;

fn main() {
    // let mut image = Image::from_dimensions(256, 256);
    let mut image = Image::from_ratio(600, 1.);

    let now = Instant::now();
    for (idx, (row, col)) in Image::walk(image.width, image.height).enumerate() {
        let pixel = &mut image.buffer[idx];
        pixel.r = (255.999 * (col as f64) / (image.height as f64)) as u8;
        pixel.g = (255.999 * (row as f64) / (image.width as f64)) as u8;
        pixel.b = (255.999 * 0.25) as u8;
    }
    println!("Rendered canvas in {}s", now.elapsed().as_secs());

    image.save("/home/css/path-tracer/out", "rainbow");
}
