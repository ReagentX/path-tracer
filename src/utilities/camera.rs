use std::f64::consts::PI;

use rand::Rng;

use crate::utilities::{image::Image, point::Point, ray::Ray};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CameraSettings {
    pub view_up: Point,
    pub position: Point,
    pub direction: Point,
    pub vertical_fov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focal_length: f64,
    pub shutter_open: f64,
    pub shutter_close: f64,
}

impl CameraSettings {
    pub fn new(
        view_up: Point,
        position: Point,
        direction: Point,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focal_length: f64,
        shutter_open: f64,
        shutter_close: f64,
    ) -> Self {
        Self {
            view_up,
            position,
            direction,
            vertical_fov,
            aspect_ratio,
            aperture,
            focal_length,
            shutter_open,
            shutter_close,
        }
    }
}

pub struct Camera {
    origin: Point,
    horizontal: Point,
    vertical: Point,
    lower_left_corner: Point,
    camera_horizontal: Point,
    camera_vertical: Point,
    lens_radius: f64,
    shutter_open: f64,
    shutter_close: f64,
}

impl Camera {
    // Create a Camera given some viewport settings
    // TODO: Make this a camera settings struct and create a camera given this data
    pub fn new(settings: &CameraSettings) -> Self {
        let theta = PI / 180. * settings.vertical_fov;

        let viewport_height = 2. * (theta / 2.).tan();
        let viewport_width = settings.aspect_ratio * viewport_height;

        let camera_distance = (settings.position - settings.direction).normalized();
        let camera_horizontal = settings.view_up.cross(camera_distance).normalized();
        let camera_vertical = camera_distance.cross(camera_horizontal);

        let horizontal = settings.focal_length * viewport_width * camera_horizontal;
        let vertical = settings.focal_length * viewport_height * camera_vertical;

        let lower_left_corner = settings.position
            - horizontal / 2.
            - vertical / 2.
            - settings.focal_length * camera_distance;

        Camera {
            origin: settings.position,
            horizontal,
            vertical,
            lower_left_corner,
            camera_horizontal,
            camera_vertical,
            lens_radius: settings.aperture / 2.0,
            shutter_open: settings.shutter_open,
            shutter_close: settings.shutter_close,
        }
    }

    pub fn default_from_image(image: &Image) -> Self {
        Self::new(
            &CameraSettings::new(
                Point::new(0., 1., 0.),
                Point::new(0., 0., 3.),
                Point::new(0., 0., -3.),
                40.,
                image.aspect_ratio(),
                0.,
                1.,
                0.,
                1.,
            )
        )
    }

    /// Create a ray from the camera towards (u, v)
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let random_in_lens = self.lens_radius * Point::random_in_disk();
        let offset =
            self.camera_horizontal * random_in_lens.x + self.camera_vertical * random_in_lens.y;
        let mut rng = rand::thread_rng();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            rng.gen_range(self.shutter_open..self.shutter_close),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            &CameraSettings::new(
                Point::new(0., 1., 0.),
                Point::origin(),
                Point::new(0., 0., -3.),
                60.,
                16. / 9.,
                0.,
                1.,
                0.,
                1.,
            )
        )
    }
}
