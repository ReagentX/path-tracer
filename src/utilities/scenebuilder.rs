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
    scene::{Scene, Settings, RenderSettings},
};

pub fn build_scene() -> Scene {
    let render_settings = RenderSettings::new(10., 10, 1., 0., 1.);
    let image = Image::widescreen(500, Landscape);
    // let image = Image::uhd(Landscape);
    // let camera = Camera::default_from_image(&image);
    let camera_settings = CameraSettings::new(
        Point::new(0., 1., 0.),
        Point::new(-10., 0., 0.),
        Point::new(10., -0.1, 0.),
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
    let world: World = vec![
        // Center
        Box::new(Sphere::new(
            Point::new(0., 0., 0.),
            Point::new(0., 0., 0.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            1.5,
            Box::new(Mirror::new(Color::gray(0.9))),
        )),
        // Closest
        Box::new(Sphere::new(
            Point::new(-3., -0.65, -3.2),
            Point::new(-3., -0.65, -3.2),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            1.,
            Box::new(Metal::new(Color::gray(0.9), 0.9)),
        )),
        // Close
        Box::new(Sphere::new(
            Point::new(-1.7, -0.56, -1.7),
            Point::new(-1.7, -0.56, -1.7),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            1.,
            Box::new(Dielectric::new(Color::rgb(0.6, 0.2, 0.9), 1.02)),
        )),
        // Far
        Box::new(Sphere::new(
            Point::new(3., -0.55, 2.5),
            Point::new(3., -0.55, 2.5),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            -1.,
            Box::new(Lambertian::new(Color::gray(0.9), 1.)),
        )),
        // Farther
        Box::new(Sphere::new(
            Point::new(4.5, -0.7, 4.5),
            Point::new(4.5, -0.7, 4.5),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            1.,
            Box::new(Mirror::new(Color::gray(0.8))),
        )),
        // Farthest
        Box::new(Sphere::new(
            Point::new(6.5, -1.1, 7.),
            Point::new(6.5, -1.1, 7.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            1.,
            Box::new(Metal::new(Color::rgb(0.1, 0.8, 0.7), 0.1)),
        )),
        // Farthest + 1
        Box::new(Sphere::new(
            Point::new(8.5, -1.45, 9.6),
            Point::new(8.5, -1.45, 9.6),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            1.,
            Box::new(Lambertian::new(Color::gray(0.7), 1.)),
        )),
        // Farthest + 2
        Box::new(Sphere::new(
            Point::new(10.5, -1.85, 12.3),
            Point::new(10.5, -1.85, 12.3),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            1.,
            Box::new(Metal::new(Color::rgb(0.8, 0.1, 0.7), 0.9)),
        )),
        // Front upper left
        Box::new(Sphere::new(
            Point::new(0., 2.1, 1.0),
            Point::new(0., 2.1, 1.3),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            0.2,
            Box::new(Metal::new(Color::rgb(1., 0.7, 0.1), 0.3)),
        )),
        // Front upper right
        Box::new(Sphere::new(
            Point::new(0., 2.4, -1.6),
            Point::new(0., 2.4, -1.4),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            0.2,
            Box::new(Metal::new(Color::rgb(0.7, 0.1, 1.), 0.7)),
        )),
        // Sun right
        Box::new(Sphere::new(
            Point::new(0., -1., 17.),
            Point::new(0., -1., 17.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            7.,
            Box::new(Light::new(Color::rgb(0.5, 0.5, 0.1), 8.)),
        )),
        // Sun left
        Box::new(Sphere::new(
            Point::new(0., -1., -17.),
            Point::new(0., -1., -17.),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            7.,
            Box::new(Light::new(Color::rgb(0.5, 0.1, 0.5), 8.)),
        )),
        // Marble
        Box::new(Sphere::new(
            Point::new(0., -1.5, 3.5),
            Point::new(0., -1.5, 3.5),
            settings.camera.shutter_open,
            settings.camera.shutter_close,
            0.2,
            Box::new(Dielectric::new(Color::gray(0.9), 1.4)),
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
    // let mut rng = rand::thread_rng();
    // for _ in 0..100 {
    //     let random_x: f64 = rng.gen_range(-70f64..70f64);
    //     let random_y: f64 = rng.gen_range(-15f64..35f64);
    //     world.push(Box::new(Sphere::new(
    //         Point::new(random_x, random_y, -100.),
    //         Point::new(random_x, random_y, -100.),
    //         settings.camera.shutter_open,
    //         settings.camera.shutter_close,
    //         0.3,
    //         Box::new(Light::new(Color::random(), 10.)),
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
