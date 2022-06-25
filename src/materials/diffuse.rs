use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, point::Point, ray::Ray},
};

pub struct Lambertian {
    alebdo: Color,
    probability: f64,
}

impl Lambertian {
    pub fn new(alebdo: Color, probability: f64) -> Self {
        Self {
            alebdo,
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
        let scattered = Ray::new(hit.point, target);

        Some((self.alebdo, scattered))
    }
}
