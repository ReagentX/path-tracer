use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, point::Point, ray::Ray},
};

pub struct Metal {
    alebdo: Color,
    probability: f64,
}

impl Metal {
    pub fn new(alebdo: Color, probability: f64) -> Self {
        Self {
            alebdo,
            probability,
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction.reflect(hit.normal).normalized();
        let scattered = Ray::new(hit.point, reflected);

        match scattered.direction.dot(hit.normal) > 0.0 {
            true => Some((self.alebdo, scattered)),
            false => None,
        }
    }
}
