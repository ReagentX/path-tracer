use crate::{
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

pub type Material = Box<dyn Scatter>;

#[typetag::serde(tag = "type")]
pub trait Scatter: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
    fn emit(&self) -> Color;
    fn random() -> Self
    where
        Self: Sized;
}
