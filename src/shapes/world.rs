use crate::{
    shapes::hit::{Hit, Hittable},
    utilities::ray::Ray,
};

use std::vec::Vec;

pub type World = Vec<Box<dyn Hittable>>;

#[typetag::serde]
impl Hittable for World {
    /// Iterate through each item in the world, returning the closest hit
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        // Stores the time it takes to hit the closest object to the camera
        // This ensures that we respect the y-axis, that is, closer objects
        // occlude farther objects
        let mut closest_so_far = time_max;

        self.iter().for_each(|shape| {
            // Determine if the collision occurred
            if let Some(contact) = shape.hit(ray, time_min, closest_so_far) {
                // Update the time to hit the closest object
                closest_so_far = contact.time;
                hit = Some(contact)
            }
        });

        hit
    }
}
