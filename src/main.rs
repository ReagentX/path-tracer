mod utilities;

use crate::utilities::{color::Color, image::Image, point::Point, ray::Ray};

use format_num::format_num;
use rayon::prelude::*;

use std::{env, time::Instant};

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    // Generate a linear gradient from max color to min color for each hue
    Color::new(
        ((1.0 - t) * u8::MAX as f64 + (t * u8::MIN as f64)) as u8,
        ((1.0 - t) * u8::MAX as f64 + (t * 120.)) as u8,
        ((1.0 - t) * u8::MAX as f64 + (t * 255.)) as u8,
        255,
    )
}

fn main() {
    // Create canvas
    let aspect_ratio = 16. / 9.;
    let mut image = Image::from_ratio(500, aspect_ratio);

    // Create camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // Tracer lens
    let origin = Point::origin();
    let horizontal = Point::new(viewport_width, 0., 0.);
    let vertical = Point::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Point::new(0., 0., focal_length);

    let now = Instant::now();
    for row in 0..image.height {
        let scanline: Vec<Color> = (0..image.width)
            // .into_par_iter() // uncomment to use multiple cores!
            .map(|col| {
                let u = (col as f64) / ((image.width - 1) as f64);
                let v = (row as f64) / ((image.height - 1) as f64);
                let r = Ray::new(
                    origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );
                ray_color(&r)
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

    image.save(
        env::current_dir().unwrap().to_str().unwrap(),
        "out/sky_gradient",
    );
}

fn rainbow() {
    // Create canvas
    let aspect_ratio = 1.;
    let mut image = Image::from_ratio(700, aspect_ratio);

    let now = Instant::now();
    for row in (0..image.height).rev() {
        let scanline: Vec<Color> = (0..image.width)
            // .into_par_iter() // uncomment to use multiple cores!
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
