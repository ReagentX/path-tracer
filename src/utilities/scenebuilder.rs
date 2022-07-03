use crate::{
    materials::{
        diffuse::Lambertian, glass::Dielectric, light::Light, metal::Metal, mirror::Mirror,
    },
    shapes::{sphere::Sphere, world::World},
};

use super::{
    camera::Camera,
    color::Color,
    image::{
        Image,
        Orientation::{Landscape, Portrait},
    },
    point::Point,
    scene::{Scene, Settings},
};

pub fn build_scene() -> Scene {
    let settings = Settings::new(10., 10, 1., 0., 1.);
    let image = Image::widescreen(500, Landscape);
    let camera = Camera::default_from_image(&image);

    // Create world
    let world: World = vec![
        // Center
        Box::new(Sphere::new(
            Point::new(0., 0., 0.),
            Point::new(0., 0., 0.),
            settings.shutter_open,
            settings.shutter_close,
            1.5,
            Box::new(Mirror::new(Color::gray(0.9))),
        )),
        // Closest
        Box::new(Sphere::new(
            Point::new(-4.5, -0.7, -3.2),
            Point::new(-4.5, -0.7, -3.2),
            settings.shutter_open,
            settings.shutter_close,
            1.,
            Box::new(Metal::new(Color::gray(0.9), 0.9)),
        )),
        // Close
        Box::new(Sphere::new(
            Point::new(-3., -0.65, -1.7),
            Point::new(-3., -0.65, -1.7),
            settings.shutter_open,
            settings.shutter_close,
            1.,
            Box::new(Dielectric::new(Color::gray(0.9), 1.5)),
        )),
        // Far
        Box::new(Sphere::new(
            Point::new(3., -0.55, 2.),
            Point::new(3., -0.55, 2.),
            settings.shutter_open,
            settings.shutter_close,
            -1.,
            Box::new(Lambertian::new(Color::gray(0.9), 1.)),
        )),
        // Farther
        Box::new(Sphere::new(
            Point::new(4.5, -0.7, 3.5),
            Point::new(4.5, -0.7, 3.5),
            settings.shutter_open,
            settings.shutter_close,
            1.,
            Box::new(Mirror::new(Color::gray(0.8))),
        )),
        // Farthest
        Box::new(Sphere::new(
            Point::new(6.5, -0.9, 5.5),
            Point::new(6.5, -0.9, 5.5),
            settings.shutter_open,
            settings.shutter_close,
            1.,
            Box::new(Metal::new(Color::rgb(0.1, 0.8, 0.7), 0.1)),
        )),
        // Farthest + 1
        Box::new(Sphere::new(
            Point::new(8.5, -1.2, 7.5),
            Point::new(8.5, -1.2, 7.5),
            settings.shutter_open,
            settings.shutter_close,
            1.,
            Box::new(Lambertian::new(Color::gray(0.7), 1.)),
        )),
        // Farthest + 2
        Box::new(Sphere::new(
            Point::new(10.5, -1.5, 9.5),
            Point::new(10.5, -1.5, 9.5),
            settings.shutter_open,
            settings.shutter_close,
            1.,
            Box::new(Metal::new(Color::rgb(0.8, 0.1, 0.7), 0.9)),
        )),
        // Front upper left
        Box::new(Sphere::new(
            Point::new(-1.2, 2.1, 1.0),
            Point::new(-1.2, 2.1, 1.3),
            settings.shutter_open,
            settings.shutter_close,
            0.2,
            Box::new(Metal::new(Color::rgb(1., 0.7, 0.1), 0.3)),
        )),
        // Front upper right
        Box::new(Sphere::new(
            Point::new(-1.2, 2.4, -1.6),
            Point::new(-1.2, 2.4, -1.4),
            settings.shutter_open,
            settings.shutter_close,
            0.2,
            Box::new(Metal::new(Color::rgb(0.7, 0.1, 1.), 0.7)),
        )),
        // Sun right
        Box::new(Sphere::new(
            Point::new(0., -1., 17.),
            Point::new(0., -1., 17.),
            settings.shutter_open,
            settings.shutter_close,
            7.,
            Box::new(Light::new(Color::rgb(0.5, 0.5, 0.1), 8.)),
        )),
        // Sun left
        Box::new(Sphere::new(
            Point::new(0., -1., -17.),
            Point::new(0., -1., -17.),
            settings.shutter_open,
            settings.shutter_close,
            7.,
            Box::new(Light::new(Color::rgb(0.5, 0.1, 0.5), 8.)),
        )),
        // Marble
        Box::new(Sphere::new(
            Point::new(-6., -1.52, 2.5),
            Point::new(-6., -1.52, 2.5),
            settings.shutter_open,
            settings.shutter_close,
            0.2,
            Box::new(Dielectric::new(Color::gray(0.9), 1.4)),
        )),
        // Ground
        Box::new(Sphere::new(
            Point::new(0., -101.5, 0.),
            Point::new(0., -101.5, 0.),
            settings.shutter_open,
            settings.shutter_close,
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
    //         settings.shutter_open,
    //         settings.shutter_close,
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
    //         settings.shutter_open,
    //         settings.shutter_close,
    //         0.3,
    //         Box::new(Light::new(Color::random(), 10.)),
    //     )));
    // }

    Scene::new(camera, settings, image, world)
}
