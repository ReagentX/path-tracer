use std::f64::consts::PI;

use rand::Rng;

use crate::utilities::{image::Image, point::Point, ray::Ray};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
        let theta = PI / 180. * vertical_fov;

        let viewport_height = 2. * (theta / 2.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let camera_distance = (position - direction).normalized();
        let camera_horizontal = view_up.cross(camera_distance).normalized();
        let camera_vertical = camera_distance.cross(camera_horizontal);

        let horizontal = focal_length * viewport_width * camera_horizontal;
        let vertical = focal_length * viewport_height * camera_vertical;

        let lower_left_corner =
            position - horizontal / 2. - vertical / 2. - focal_length * camera_distance;

        Camera {
            origin: position,
            horizontal,
            vertical,
            lower_left_corner,
            camera_horizontal,
            camera_vertical,
            lens_radius: aperture / 2.0,
            shutter_open,
            shutter_close,
        }
    }

    pub fn default_from_image(image: &Image) -> Self {
        Self::new(
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
    }
}
