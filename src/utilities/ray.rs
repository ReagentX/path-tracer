use crate::utilities::point::Point;

pub struct Ray {
    origin: Point,
    direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, time: f64) -> Point {
        self.origin + time * self.direction
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::{point::Point, ray::Ray};

    #[test]
    fn can_create() {
        let p1 = Point::new(0., 0., 0.);
        let p2 = Point::new(5., 5., 5.);
        let ray = Ray::new(p1, p2);
        assert_eq!(ray.origin.x, 0.);
        assert_eq!(ray.origin.y, 0.);
        assert_eq!(ray.origin.z, 0.);
        assert_eq!(ray.direction.x, 5.);
        assert_eq!(ray.direction.y, 5.);
        assert_eq!(ray.direction.z, 5.);
    }

    #[test]
    fn can_get_at() {
        let p1 = Point::new(0., 0., 0.);
        let p2 = Point::new(5., 5., 5.);
        let ray = Ray::new(p1, p2);
        let ray_at_t1 = ray.at(0.5);
        assert_eq!(ray_at_t1.x, 2.5);
        assert_eq!(ray_at_t1.y, 2.5);
        assert_eq!(ray_at_t1.z, 2.5);
    }
}
