use crate::{
    materials::scatter::Material,
    shapes::hit::{Hit, Hittable},
    utilities::{point::Point, ray::Ray},
};

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
    material: Material,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point, material: Material) -> Self {
        Self { a, b, c, material }
    }
}

impl Hittable for Triangle {
    /// Implemented using the Möller–Trumbore intersection algorithm using the two-sided approach
    ///
    /// https://cadxfem.org/inf/Fast%20MinimumStorage%20RayTriangle%20Intersection.pdf
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<Hit> {
        let edge_1 = self.b - self.a;
        let edge_2 = self.c - self.a;

        /*
        I don't know what this value from the paper is, but it is used
        to calcualte the determinant and u coordinate inside of the triangle
        */
        let p_vec = ray.direction.cross(edge_2);

        // Calculate determinant
        let determinant = edge_1.dot(p_vec);

        // If the determinant is near zero, the ray is parallel to the triangle
        if determinant > -f64::EPSILON && determinant < f64::EPSILON {
            return None;
        }

        // Calculate inverse determinant
        let inverse_determinant = 1. / determinant;

        // Distance from point a to ray origin
        let t_vec = ray.origin - self.a;

        /*
        (u, v) are the coordinates inside the triangle
        This is the u component
        */
        let u = t_vec.dot(p_vec) * inverse_determinant;

        /*
        The value of u is compared to an edge of the triangle (u=0)
        and also to a line parallel to that edge but passing through
        the opposite p oint of the triangle (u=1). This test rules
        out many intersection points ahead of time
        */
        if !(0. ..=1.).contains(&u) {
            return None;
        }

        /*
        I don't know what this value from the paper is, but it is used
        to calcualte the v coordinate inside of the triangle
        */
        let q_vec = t_vec.cross(edge_1);
        // This is the v component
        let v = ray.direction.dot(q_vec) * inverse_determinant;

        // v follows the same rule as u
        if v < 0.0 || u + v > 1.0  {
            return None;
        }

        // If we got this far, the ray intersects the triangle at point (time, u, v)
        let time = edge_2.dot(q_vec) * inverse_determinant;

        if time > f64::EPSILON {
            // Calculate the surface normal
            let normal = edge_2.cross(edge_1).normalized();

            return Some(Hit::new(ray.at(time), normal, &self.material, time, false));
        }

        
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        materials::{diffuse::Lambertian, scatter::Scatter},
        shapes::{hit::Hittable, triangle::Triangle},
        utilities::{point::Point, ray::Ray},
    };

    #[test]
    fn can_create() {
        let mat = Lambertian::random();
        let _t = Triangle::new(
            Point::new(0., 3., 1.),
            Point::new(-2., 0., 1.),
            Point::new(0., -2., 1.),
            Box::new(mat),
        );
    }

    #[test]
    fn can_hit() {
        let mat = Lambertian::random();
        let t = Triangle::new(
            Point::new(2., 3., 1.),
            Point::new(-2., -1., 1.),
            Point::new(-1., -2., 1.),
            Box::new(mat),
        );

        assert!(t
            .hit(&Ray::new(Point::origin(), Point::new(0., 0., 2.)), 0., 3.,)
            .is_some());
    }
}
