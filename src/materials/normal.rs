use rand::Rng;

use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Normal {
    brightness: f64,
    intensity: f64,
}

impl Normal {
    pub fn new(brightness: f64, intensity: f64) -> Self {
        Self {
            brightness,
            intensity,
        }
    }
}

#[typetag::serde]
impl Scatter for Normal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let ray_out = Ray::new(hit.point, ray_in.direction, ray_in.time);
        let color = Color::rgb(
            self.brightness * (hit.normal.x + self.intensity),
            self.brightness * (hit.normal.y + self.intensity),
            self.brightness * (hit.normal.z + self.intensity),
        );
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
            brightness: rng.gen_range(1.0..2.0),
            intensity: rng.gen_range(1.0..2.0),
        }
    }
}
