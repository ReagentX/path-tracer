use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

pub struct None {}

impl None {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scatter for None {
    fn scatter(&self, _: &Ray, _: &Hit) -> Option<(Color, Ray)> {
        None
    }
}
