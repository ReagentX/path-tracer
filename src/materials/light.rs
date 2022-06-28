use rand::Rng;

use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, point::Point, ray::Ray},
};

pub struct Light {
    albedo: Color,
    intensity: f64,
}

impl Light {
    pub fn new(albedo: Color, intensity: f64) -> Self {
        Self { albedo, intensity }
    }
}

impl Scatter for Light {
    fn scatter(&self, _: &Ray, _: &Hit) -> Option<(Color, Ray)> {
        None
    }

    fn emit(&self) -> Color {
        self.albedo * self.intensity
    }

    fn random() -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();
        Self {
            albedo: Color::random(),
            intensity: rng.gen_range(1.0..10.0),
        }
    }
}
