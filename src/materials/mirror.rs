use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

pub struct Mirror {
    albedo: Color,
}

impl Mirror {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Mirror {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction.reflect(hit.normal).normalized();
        let scattered = Ray::new(hit.point, reflected, ray_in.time);

        match scattered.direction.dot(hit.normal) > 0.0 {
            true => Some((self.albedo, scattered)),
            false => None,
        }
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
            albedo: Color::random(),
        }
    }
}
