use crate::utilities::{point::Point, ray::Ray};

pub struct Hit {
    pub point: Point,
    pub normal: Point,
    pub time: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(point: Point, normal: Point, time: f64, front_face: bool) -> Self {
        Hit {
            point,
            normal,
            time,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Point) {
        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = match self.front_face {
            true => outward_normal,
            false => outward_normal * -1.,
        }
    }
}

pub trait Hittable: Send + Sync  {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<Hit>;
}
