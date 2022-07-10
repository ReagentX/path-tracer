#![forbid(unsafe_code)]

mod materials;
mod shapes;
mod utilities;

use crate::{
    shapes::{hit::Hittable, world::World},
    utilities::{
        color::Color, image::Image, progress::build_progress_bar, ray::Ray, scene::Scene,
        scenebuilder::build_scene,
    },
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
        // Color::default()
    }
}

fn main() {
    let mut scene = Scene::load(
        env::current_dir().unwrap().to_str().unwrap(),
        "scenes/test",
    );
    // let mut scene = build_scene();
    // scene.save(env::current_dir().unwrap().to_str().unwrap(), "scenes/iwp");

    let pb = build_progress_bar(scene.image.pixels());
    let mut pixels_rendered = 0;
    let now = Instant::now();

    // Iterate through each row in parallel, collecting each row to write to the image
    for row in 0..scene.image.height {
        let scanline: Vec<Color> = (0..scene.image.width)
            .into_par_iter()
            .map(|col| {
                // Collect color samples for MSAA
                let mut red_component = 0.;
                let mut blue_component = 0.;
                let mut green_component = 0.;

                // Generate random rays for each pixel
                let mut rng = rand::thread_rng();
                for _ in 0..scene.settings.render.msaa_samples as u64 {
                    // Get random endpoint
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    // Create valid (u, v) direction for ray
                    let u = ((col as f64) + random_u) / ((scene.image.width - 1) as f64);
                    let v = ((row as f64) + random_v) / ((scene.image.height - 1) as f64);
                    let r = scene.camera.get_ray(u, v);

                    // Get the pixel color
                    let pixel = ray_color(&r, &scene.world, scene.settings.render.max_depth);
                    red_component += pixel.r;
                    green_component += pixel.g;
                    blue_component += pixel.b;
                }

                Color::rgb(
                    red_component / scene.settings.render.msaa_samples,
                    green_component / scene.settings.render.msaa_samples,
                    blue_component / scene.settings.render.msaa_samples,
                )
            })
            .collect();

        // Emit progress
        pixels_rendered += scene.image.width;
        pb.set_position(pixels_rendered);

        // Write the scanline to the image
        for (col, pixel) in scanline.iter().enumerate() {
            *scene.image.color_at(col as u64, row) = *pixel;
        }
    }
    pb.finish_at_current_pos();

    // Print metrics
    let elapsed = now.elapsed().as_millis();
    println!(
        "Rendered canvas in {:.2}s ({} pixels per milisecond)",
        elapsed as f64 / 1000.,
        format_num!(",d", scene.image.buffer.len() as f64 / elapsed as f64)
    );

    scene.render(
        env::current_dir().unwrap().to_str().unwrap(),
        "render/sky_gradient",
    );
}
