use rand::Rng;

use crate::{
    materials::{
        diffuse::Lambertian, glass::Dielectric, light::Light, metal::Metal, mirror::Mirror,
    },
    shapes::{sphere::Sphere, triangle::Triangle, world::World},
};

use super::{
    camera::{Camera, CameraSettings},
    color::Color,
    image::{
        Image,
        Orientation::{Landscape, Portrait},
    },
    point::Point,
    scene::{RenderSettings, Scene, Settings},
};

pub fn build_scene() -> Scene {
    let render_settings = RenderSettings::new(10., 5, 1., 0., 1.);
    let image = Image::square(1000);

    // let image = Image::hd(Landscape);
    // let camera = Camera::default_from_image(&image);
    let camera_settings = CameraSettings::new(
        Point::new(0., 1., 0.),
        Point::new(0., 0., 5.),
        Point::new(0., 1., -2.),
        100.,
        image.aspect_ratio(),
        0.,
        45.,
        0.,
        1.,
    );
    let camera = Camera::new(&camera_settings);

    let settings = Settings::new(render_settings, camera_settings);

    // Create world

    let world: World = vec![
        // Right marble
        Box::new(Sphere::new(
            Point::new(0.5, 0., 1.5),
            Point::new(0.5, 0., 1.5),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            0.2,
            Box::new(Dielectric::new(Color::gray(1.), 1.9)),
        )),
        // Left lamp
        Box::new(Sphere::new(
            Point::new(-2.2, -1.5, 0.3),
            Point::new(-2.2, -1.5, 0.3),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            0.35,
            Box::new(Light::new(Color::gray(1.), 250.)),
        )),
        // Left Triangle
        Box::new(Triangle::new(
            Point::new(-5., -3., -5.),
            Point::new(0., -4., 0.),
            Point::new(0., 5., -5.),
            // Box::new(Mirror::new(Color::gray(1.))),
            Box::new(Lambertian::new(Color::rgb(1., 0., 0.), 0.5)),
        )),
        // Right Triangle
        Box::new(Triangle::new(
            Point::new(5., -3., -5.),
            Point::new(0., -4., 0.),
            Point::new(0., 5., -5.),
            // Box::new(Mirror::new(Color::gray(1.))),
            Box::new(Lambertian::new(Color::rgb(0., 1., 0.), 0.5)),
        )),
        // Sun
        Box::new(Sphere::new(
            Point::new(0., 50., 0.),
            Point::new(0., 50., 0.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            5.,
            Box::new(Light::new(Color::gray(1.), 20.)),
        )),
        // Ground
        Box::new(Sphere::new(
            Point::new(0., -101.5, 0.),
            Point::new(0., -101.5, 0.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            100.,
            Box::new(Lambertian::new(Color::rgb(0.4, 0.3, 0.3), 1.)),
        )),
    ];

    // Add stars
    // let mut rng = rand::thread_rng();
    // for _ in 0..300 {
    //     let random_x: f64 = rng.gen_range(-55f64..55f64);
    //     let random_y: f64 = rng.gen_range(-80f64..80f64);
    //     world.push(Box::new(Sphere::new(
    //         Point::new(random_x, random_y, -100. + (random_x.abs() / 2.)),
    //         Point::new(random_x, random_y, -100. + (random_x.abs() / 2.)),
    //         settings.camera.shutter_open,
    //         settings.camera.shutter_close,
    //         0.3,
    //         Box::new(Light::new(Color::random(), 5.)),
    //     )));
    // }
    // for _ in 0..50 {
    //     let mut rng = rand::thread_rng();
    //     let random_x: f64 = rng.gen_range(-300f64..300f64);
    //     let random_y: f64 = rng.gen_range(-10f64..200f64);
    //     world.push(Box::new(Sphere::new(
    //         Point::new(random_x, random_y, 50.),
    //         Point::new(random_x, random_y, 50.),
    //         settings.camera.shutter_open,
    //         settings.camera.shutter_close,
    //         0.3,
    //         Box::new(Light::new(Color::random(), 10.)),
    //     )));
    // }

    Scene::new(settings, image, camera, world)
}
