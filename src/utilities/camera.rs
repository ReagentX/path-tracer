use crate::utilities::{image::Image, point::Point, ray::Ray};

pub struct Camera {
    origin: Point,
    horizontal: Point,
    vertical: Point,
    lower_left_corner: Point,
}

impl Camera {
    // Create a Camera for a given image
    pub fn from_image(image: &Image, focal_length: f64, position: Point) -> Self {
        let aspect_ratio = image.width as f64 / image.height as f64;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let lens_position = position;
        let horizontal = Point::new(viewport_width, 0., 0.);
        let vertical = Point::new(0., viewport_height, 0.);
        let lower_left_corner =
            lens_position - horizontal / 2. - vertical / 2. - Point::new(0., 0., focal_length);
        Camera {
            origin: lens_position,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    /// Create a ray from the camera towards (u, v)
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
