#![forbid(unsafe_code)]

mod materials;
mod shapes;
mod utilities;

use crate::{
    materials::{
        diffuse::Lambertian, glass::Dielectric, light::Light, metal::Metal, mirror::Mirror,
        normal::Normal, scatter::Scatter, transparent::Filter,
    },
    shapes::{hit::Hittable, sphere::Sphere, world::World},
    utilities::{camera::Camera, color::Color, image::Image, point::Point, ray::Ray},
};

use format_num::format_num;
use rand::Rng;
use rayon::prelude::*;

use std::{env, time::Instant};

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        // Hit, generate a color using the material
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            hit.material.emit()
        }
    } else {
        // Miss, generate sky
        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        // Generate a linear gradient from max color to min color for each hue
        (1.0 - t) * Color::gray(1.0) + t * Color::rgb(0.5, 0.5, 0.9)
    }
}

fn main() {
    // Create canvas
    let mut image = Image::widescreen(500);

    // Samples for MSAA
    const SAMPLES: f64 = 100.0;
    // Number of bounces before a ray dies
    const MAX_DEPTH: u64 = 100;
    // Gamma
    const GAMMA: f64 = 1.;

    // Create world
    let world: World = vec![
        // Back
        Box::new(Sphere::new(
            Point::new(3.1, -1.6, -8.),
            0.6,
            Box::new(Light::random()),
        )),
        // Center
        Box::new(Sphere::new(
            Point::new(0., 0., -5.),
            -1.5,
            Box::new(Mirror::random()),
        )),
        // Center left
        Box::new(Sphere::new(
            Point::new(-3.3, -0.08, -5.2),
            1.5,
            Box::new(Metal::random()),
        )),
        // Center right
        Box::new(Sphere::new(
            Point::new(3.3, -0.08, -5.2),
            -1.5,
            Box::new(Dielectric::random()),
        )),
        // Front upper right bubble
        Box::new(Sphere::new(
            Point::new(1.2, 1.3, -3.8),
            -0.5,
            Box::new(Dielectric::random()),
        )),
        // Sun
        Box::new(Sphere::new(
            Point::new(0., -1., 15.),
            7.,
            Box::new(Light::random()),
        )),
        // Front left
        Box::new(Sphere::new(
            Point::new(-1.2, -1.35, -4.),
            0.2,
            Box::new(Dielectric::random()),
        )),
        // Front right
        Box::new(Sphere::new(
            Point::new(1.2, -1.5, -4.),
            0.2,
            Box::new(Metal::random()),
        )),
        // Ground
        Box::new(Sphere::new(
            Point::new(0., -101.5, -2.),
            100.,
            Box::new(Lambertian::random()),
        )),
    ];

    // Create camera
    let camera = Camera::from_image(&image, 2.0, Point::origin());

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

                Color::rgb(
                    red_component / SAMPLES,
                    green_component / SAMPLES,
                    blue_component / SAMPLES,
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
                Color::rgb(
                    (255.0 * col as f64) / image.width as f64,
                    (255.0 * row as f64) / image.height as f64,
                    63.0,
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
