use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

pub struct Normal {
    brightness: f64,
}

impl Normal {
    pub fn new(brightness: f64) -> Self {
        Self { brightness }
    }
}

impl Scatter for Normal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let ray_out = Ray::new(hit.point, ray_in.direction);
        let color = Color::rgb(
            self.brightness * (hit.normal.x + 1.),
            self.brightness * (hit.normal.y + 1.),
            self.brightness * (hit.normal.z + 1.),
        );
        Some((color, ray_out))
    }
}
