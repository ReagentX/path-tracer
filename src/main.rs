mod utilities;

use crate::utilities::{color::Color, image::Image};
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // let mut image = Image::from_dimensions(256, 256);
    let mut image = Image::from_ratio(600, 1.);

    let now = Instant::now();
    for row in (0..image.height).rev() {
        let scanline: Vec<Color> = (0..image.width)
            .into_par_iter()
            .map(|col| {
                Color::new(
                    ((255 * col) / image.width) as u8,
                    ((255 * row) / image.height) as u8,
                    63_u8,
                    255,
                )
            })
            .collect();

        for (col, pixel) in scanline.iter().enumerate() {
            *image.color_at(row, col as u64) = *pixel;
        }
    }

    // Print metrics
    let elapsed = now.elapsed().as_millis();
    println!(
        "Rendered canvas in {:.2}s ({:.0} pixels per milisecond)",
        elapsed as f64 / 1000.,
        image.buffer.len() as f64 / elapsed as f64
    );

    image.save("/home/css/path-tracer/out", "rainbow");
}
