use rand::Rng;

use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Light {
    albedo: Color,
    intensity: f64,
}

impl Light {
    pub fn new(albedo: Color, intensity: f64) -> Self {
        Self { albedo, intensity }
    }
}

#[typetag::serde]
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
