use rand::Rng;

use crate::{
    materials::{
        diffuse::Lambertian, glass::Dielectric, light::Light, metal::Metal, mirror::Mirror,
    },
    shapes::{sphere::Sphere, world::World},
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
    let render_settings = RenderSettings::new(10., 10, 1., 0., 1.);
    let image = Image::mobile(3, Portrait);
    // let image = Image::uhd(Landscape);
    // let camera = Camera::default_from_image(&image);
    let camera_settings = CameraSettings::new(
        Point::new(0., 1., 0.),
        Point::new(0., 0., 3.),
        Point::new(0., 0., -2.),
        45.,
        image.aspect_ratio(),
        0.,
        40.,
        0.,
        1.,
    );
    let camera = Camera::new(&camera_settings);

    let settings = Settings::new(render_settings, camera_settings);

    // Create world
    let mut world: World = vec![
        // Center
        Box::new(Sphere::new(
            Point::new(0., -0.28, -6.),
            Point::new(0., -0.28, -6.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            1.4,
            Box::new(Dielectric::new(Color::gray(0.9), 1.5)),
        )),
        // Light
        Box::new(Sphere::new(
            Point::new(0., 10., -4.),
            Point::new(0., 10., -4.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            5.,
            Box::new(Light::new(Color::gray(1.), 4.)),
        )),
        // Ground
        Box::new(Sphere::new(
            Point::new(0., -101.5, 0.),
            Point::new(0., -101.5, 0.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            100.,
            Box::new(Lambertian::new(Color::rgb(0.9, 0.2, 0.4), 1.)),
        )),
    ];

    // Add stars
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let random_x: f64 = rng.gen_range(-55f64..55f64);
        let random_y: f64 = rng.gen_range(-20f64..75f64);
        world.push(Box::new(Sphere::new(
            Point::new(random_x, random_y, -100. + (random_x.abs() / 2.)),
            Point::new(random_x, random_y, -100. + (random_x.abs() / 2.)),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            0.3,
            Box::new(Light::new(Color::random(), 5.)),
        )));
    }
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
