use crate::{
    shapes::hit::{Hit, Hittable},
    utilities::{point::Point, ray::Ray},
};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
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

        let mut hit = Hit::new(ray.at(root), Point::origin(), root, false);
        let outward_normal = (hit.point - self.center) / self.radius;
        hit.set_face_normal(ray, outward_normal);
        Some(hit)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{
            hit::{Hittable},
            sphere::Sphere,
        },
        utilities::{point::Point, ray::Ray},
    };

    #[test]
    fn can_create() {
        let s = Sphere::new(Point::origin(), 1.0);
        assert_eq!(s.radius, 1.0);
        assert_eq!(s.center.x, Point::default().x);
        assert_eq!(s.center.y, Point::default().y);
        assert_eq!(s.center.z, Point::default().z);
    }

    #[test]
    fn can_hit() {
        let s = Sphere::new(Point::origin(), 1.0);
        assert!(s
            .hit(&Ray::new(Point::origin(), Point::new(0., 0., -1.)), 0., 1.,)
            .is_some());
    }

    #[test]
    fn can_miss() {
        let s = Sphere::new(Point::origin(), 1.0);
        assert!(s
            .hit(
                &Ray::new(Point::new(3., 3., -1.), Point::new(3., 3., -1.)),
                0.,
                1.,
            )
            .is_none());
    }
}
