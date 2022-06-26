use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

pub struct Filter {
    alebdo: Color,
    opacity: f64,
}

// TODO: A proper transparent material needs to know the color of the incoming ray
impl Filter {
    pub fn new(alebdo: Color, opacity: f64) -> Self {
        Self { alebdo, opacity }
    }
}

impl Scatter for Filter {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let ray_out = Ray::new(hit.point, ray_in.direction);
        let color = self.opacity * self.alebdo;
        Some((color, ray_out))
    }

    fn emit(&self) -> Color {
        Color::default()
    }
}
