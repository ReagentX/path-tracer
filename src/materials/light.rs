use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, point::Point, ray::Ray},
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
    fn scatter(&self, _: &Ray, _: &Hit) -> Option<(Color, Ray)> {
        // let ray_out = Ray::new(
        //     hit.point,
        //     ray_in.direction + Point::random_in_sphere().normalized(),
        // );
        // Some(((self.alebdo * (self.intensity / hit.point.len())), ray_out))
        None
    }

    fn emit(&self) -> Color {
        self.alebdo * self.intensity
    }
}
