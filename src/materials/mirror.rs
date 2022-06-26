use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

pub struct Mirror {
    alebdo: Color,
}

impl Mirror {
    pub fn new(alebdo: Color) -> Self {
        Self { alebdo }
    }
}

impl Scatter for Mirror {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction.reflect(hit.normal).normalized();
        let scattered = Ray::new(hit.point, reflected);

        match scattered.direction.dot(hit.normal) > 0.0 {
            true => Some((self.alebdo, scattered)),
            false => None,
        }
    }

    fn emit(&self) -> Color {
        Color::default()
    }
}
