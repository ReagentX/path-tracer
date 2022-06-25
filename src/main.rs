#![forbid(unsafe_code)]

mod shapes;
mod utilities;

use crate::{
    shapes::{hit::Hittable, sphere::Sphere, world::World},
    utilities::{camera::Camera, color::Color, image::Image, point::Point, ray::Ray},
};

use format_num::format_num;
use rand::Rng;
use rayon::prelude::*;

use std::{env, time::Instant};

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Color {
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if depth == 0 {
            return Color::default();
        }

        // Hit, use normal vector to generate a color
        // Color::new(
        //     0.5 * (hit.normal.x + 1.),
        //     0.5 * (hit.normal.y + 1.),
        //     0.5 * (hit.normal.z + 1.),
        //     255,
        // )
        // Hit, use random direction to generate a color
        let target = hit.point + hit.normal + Point::random_in_sphere().normalized();
        let random_ray = Ray::new(hit.point, target - hit.point);
        0.5 * ray_color(&random_ray, world, depth - 1)
    } else {
        // Miss, generate sky
        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        // Generate a linear gradient from max color to min color for each hue
        (1.0 - t) * Color::new(1.0, 1.0, 1.0, 255) + t * Color::new(0.5, 0.7, 1.0, 255)
    }
}

fn main() {
    // Create canvas
    let aspect_ratio = 16. / 9.;
    let mut image = Image::from_ratio(500, aspect_ratio);

    // Samples for MSAA
    const SAMPLES: f64 = 100.0;
    // Number of bounces before a ray dies
    const MAX_DEPTH: u64 = 10;
    // Gamma
    const GAMMA: f64 = 1.1;

    // Create world
    let world: World = vec![
        Box::new(Sphere::new(Point::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Point::new(0., -100.5, -2.), 100.)),
    ];

    // Create camera
    let camera = Camera::from_image(&image);

    let now = Instant::now();
    for row in 0..image.height {
        let scanline: Vec<Color> = (0..image.width)
            .into_par_iter() // drop in to use multiple cores!
            .map(|col| {
                // Collect color samples for MSAA
                let mut red_component = 0.;
                let mut blue_component = 0.;
                let mut green_component = 0.;

                // Generate random rays for each pixel
                for _ in 0..SAMPLES as u64 {
                    // Get random endpoint
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    // Create valid (u, v) direction for ray
                    let u = ((col as f64) + random_u) / ((image.width - 1) as f64);
                    let v = ((row as f64) + random_v) / ((image.height - 1) as f64);
                    let r = camera.get_ray(u, v);

                    // Get the pixel color
                    let pixel = ray_color(&r, &world, MAX_DEPTH);
                    red_component += pixel.r;
                    green_component += pixel.g;
                    blue_component += pixel.b;
                }

                Color::new(
                    red_component / SAMPLES,
                    green_component / SAMPLES,
                    blue_component / SAMPLES,
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

    image.save(
        env::current_dir().unwrap().to_str().unwrap(),
        "out/sky_gradient",
        GAMMA,
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
                    (255.0 * col as f64) / image.width as f64,
                    (255.0 * row as f64) / image.height as f64,
                    63.0,
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

    image.save(
        env::current_dir().unwrap().to_str().unwrap(),
        "out/rainbow",
        1.,
    );
}
