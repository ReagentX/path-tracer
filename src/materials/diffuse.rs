use rand::Rng;

use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, point::Point, ray::Ray},
};

pub struct Lambertian {
    albedo: Color,
    probability: f64,
}

impl Lambertian {
    pub fn new(albedo: Color, probability: f64) -> Self {
        Self {
            albedo,
            probability,
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let mut target = hit.normal + Point::random_in_sphere().normalized();
        if target.is_near_zero() {
            target = hit.normal
        }
        let scattered = Ray::new(hit.point, target, ray_in.time);

        Some((self.albedo, scattered))
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
            probability: rng.gen_range(0.0..1.0),
        }
    }
}
