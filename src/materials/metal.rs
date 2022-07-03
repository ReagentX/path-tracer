use rand::Rng;

use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, point::Point, ray::Ray},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Metal {
    albedo: Color,
    /// 0..1 range of matte, higher means less reflective
    matte: f64,
}

impl Metal {
    pub fn new(albedo: Color, matte: f64) -> Self {
        Self { albedo, matte }
    }
}

#[typetag::serde]
impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction.reflect(hit.normal).normalized();
        let scattered = Ray::new(
            hit.point,
            reflected + self.matte * Point::random_in_sphere(), ray_in.time
        );

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
            matte: rng.gen_range(0.1..1.0),
        }
    }
}
