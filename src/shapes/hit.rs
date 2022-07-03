use crate::{
    materials::scatter::Material,
    utilities::{point::Point, ray::Ray},
};

pub struct Hit<'a> {
    pub point: Point,
    pub normal: Point,
    pub material: &'a Material,
    pub time: f64,
    pub front_face: bool,
}

impl<'a> Hit<'a> {
    pub fn new(
        point: Point,
        normal: Point,
        material: &'a Material,
        time: f64,
        front_face: bool,
    ) -> Self {
        Hit {
            point,
            normal,
            material,
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

#[typetag::serde(tag = "type")]
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<Hit>;
}
