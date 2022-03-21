mod utilities;

use crate::utilities::{color::Color, image::Image};
use format_num::format_num;
use rayon::prelude::*;
use std::{env, time::Instant};

fn main() {
    // let mut image = Image::from_dimensions(256, 256);
    let mut image = Image::from_ratio(700, 1.);

    let now = Instant::now();
    for row in (0..image.height).rev() {
        let scanline: Vec<Color> = (0..image.width)
            // .into_par_iter()
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
            *image.color_at(col as u64, row) = *pixel;
        }
    }

    // Print metrics
    let elapsed = now.elapsed().as_millis();
    println!(
        "Rendered canvas in {:.2}s ({} pixels per milisecond)",
        elapsed as f64 / 1000.,
        format_num!(",d", image.buffer.len() as f64 / elapsed as f64)
    );

    image.save(env::current_dir().unwrap().to_str().unwrap(), "out/rainbow");
}
