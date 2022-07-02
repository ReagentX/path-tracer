use rand::Rng;

use crate::{
    materials::scatter::Scatter,
    shapes::hit::Hit,
    utilities::{color::Color, ray::Ray},
};

pub struct Dielectric {
    albedo: Color,
    /// air = 1.0, glass = 1.3–1.7, diamond = 2.4
    /// Higher indeces mean more refractive effects
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(albedo: Color, refraction_index: f64) -> Self {
        Self {
            albedo,
            refraction_index,
        }
    }

    /// Schlick's approximation for reflectance
    /// https://en.wikipedia.org/wiki/Schlick's_approximation
    pub fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        let r_0 = ((1. - refraction_ratio) / (1. + refraction_ratio)).powi(2);
        r_0 + (1. - r_0) * (1. - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let refraction_ratio = hit
            .front_face
            .then(|| 1. / self.refraction_index)
            .unwrap_or_else(|| self.refraction_index);

        let unit_direction = ray_in.direction.normalized();
        let cos_theta = (-1. * unit_direction).dot(hit.normal).min(1.);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = (cannot_refract || will_reflect)
            .then(|| unit_direction.reflect(hit.normal))
            .unwrap_or_else(|| unit_direction.refract(hit.normal, refraction_ratio));

        let scattered = Ray::new(hit.point, direction);
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
            albedo: Color::gray(rng.gen_range(0.5..1.0)),
            refraction_index: rng.gen_range(-1.0..2.0),
        }
    }
}
