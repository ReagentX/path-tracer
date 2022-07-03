use crate::{
    materials::scatter::Material,
    shapes::hit::{Hit, Hittable},
    utilities::{point::Point, ray::Ray},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    center_t_0: Point,
    center_t_1: Point,
    t_0: f64,
    t_1: f64,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(
        center_t_0: Point,
        center_t_1: Point,
        t_0: f64,
        t_1: f64,
        radius: f64,
        material: Material,
    ) -> Self {
        Sphere {
            center_t_0,
            center_t_1,
            t_0,
            t_1,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point {
        if self.center_t_0 == self.center_t_1 {
            return self.center_t_0;
        }
        self.center_t_0
            + ((time - self.t_0) / (self.t_1 - self.t_0) * (self.center_t_1 - self.center_t_0))
    }
}

#[typetag::serde]
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.len().powi(2);
        let half_b = oc.dot(ray.direction);
        let c = oc.len().powi(2) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0. {
            return None;
        }

        // Find the nearest root given the min..max requirement
        let sqrt_disc = discriminant.sqrt();
        let mut root = (-half_b - sqrt_disc) / a;
        if root < time_min || time_max < root {
            root = (-half_b + sqrt_disc) / a;
            if root < time_min || time_max < root {
                return None;
            }
        }

        let mut hit = Hit::new(ray.at(root), Point::origin(), &self.material, root, false);
        let outward_normal = (hit.point - self.center(ray.time)) / self.radius;
        hit.set_face_normal(ray, outward_normal);
        Some(hit)
    }
}

#[cfg(test)]
mod stationary_tests {
    use crate::{
        materials::diffuse::Lambertian,
        shapes::{hit::Hittable, sphere::Sphere},
        utilities::{color::Color, point::Point, ray::Ray},
    };

    #[test]
    fn can_create() {
        let mat = Lambertian::new(Color::random(), 1.0);
        let s = Sphere::new(Point::origin(), Point::origin(), 0., 0., 1.0, Box::new(mat));
        assert_eq!(s.radius, 1.0);
        assert_eq!(s.center_t_0.x, Point::default().x);
        assert_eq!(s.center_t_0.y, Point::default().y);
        assert_eq!(s.center_t_0.z, Point::default().z);
    }

    #[test]
    fn can_hit() {
        let mat = Lambertian::new(Color::random(), 1.0);
        let s = Sphere::new(Point::origin(), Point::origin(), 0., 0., 1.0, Box::new(mat));
        assert!(s
            .hit(
                &Ray::new(Point::origin(), Point::new(0., 0., -1.), 0.),
                0.,
                1.,
            )
            .is_some());
    }

    #[test]
    fn can_miss() {
        let mat = Lambertian::new(Color::random(), 1.0);
        let s = Sphere::new(Point::origin(), Point::origin(), 0., 0., 1.0, Box::new(mat));
        assert!(s
            .hit(
                &Ray::new(Point::new(3., 3., -1.), Point::new(3., 3., -1.), 0.),
                0.,
                1.,
            )
            .is_none());
    }
}
