use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

pub struct Light {
    alebdo: Color,
    intensity: f64,
}

impl Light {
    pub fn new(alebdo: Color, intensity: f64) -> Self {
        Self { alebdo, intensity }
    }
}

impl Scatter for Light {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let ray_out = Ray::new(hit.point, ray_in.direction);
        Some(((self.alebdo * (self.intensity / hit.point.len())), ray_out))
    }
}
