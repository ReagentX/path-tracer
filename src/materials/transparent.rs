use rand::Rng;

use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Filter {
    albedo: Color,
    opacity: f64,
}

impl Filter {
    pub fn new(albedo: Color, opacity: f64) -> Self {
        Self { albedo, opacity }
    }
}

#[typetag::serde]
impl Scatter for Filter {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let ray_out = Ray::new(hit.point, ray_in.direction, ray_in.time);
        let color = self.opacity * self.albedo;
        Some((color, ray_out))
    }

    fn emit(&self) -> Color {
        Color::default()
    }

    fn random() -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();
        Self {
            albedo: Color::default(),
            opacity: rng.gen_range(0.1..2.0),
        }
    }
}
