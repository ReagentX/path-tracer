use crate::shapes::hit::{Hit, Hittable};

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    /// Iterate through each item in the world, returning the closest hit
    fn hit(&self, ray: &crate::utilities::ray::Ray, time_min: f64, time_max: f64) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut closest_so_far = time_max;

        self.iter().for_each(|shape| {
            if let Some(contact) = shape.hit(ray, time_min, time_max) {
                closest_so_far = contact.time;
                hit = Some(contact)
            }
        });

        hit
    }
}
