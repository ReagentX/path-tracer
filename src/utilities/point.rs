use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign};

use rand::distributions::{Distribution, Uniform};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    /// Alias for default, (0, 0, 0)
    pub fn origin() -> Self {
        Point::default()
    }

    /// Dot product of 2 vectors
    /// https://en.wikipedia.org/wiki/Dot_product
    pub fn dot(self, rhs: Point) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// A vector dotted with itself is equal to the squared length of that vector
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Cross product of two vectors
    /// https://en.wikipedia.org/wiki/Cross_product
    pub fn cross(self, rhs: Point) -> Point {
        Point::new(
            self.y * rhs.z + self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// Scale point to unit length (-1..1)
    pub fn normalized(self) -> Point {
        self / self.len()
    }

    /// Generate a random point inside of a given uniform distribution
    pub fn random(range: Range<f64>) -> Point {
        let between = Uniform::from(range);
        let mut rng = rand::thread_rng();
        Point::new(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
        )
    }

    /// Pick a random point in a unit radius sphere, rejecting points outside of the sphere
    /// https://en.wikipedia.org/wiki/Unit_sphere
    pub fn random_in_sphere() -> Point {
        // Create a point
        let mut point = Self::random(-1.0..1.0);
        while point.len() > 1.0 {
            point = Self::random(-1.0..1.0);
        }
        point
    }

    /// Determine if a point is near to 0 in all dimensions
    pub fn is_near_zero(&self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }

    /// Reflect a point given a surface normal ray impact
    pub fn reflect(self, normal: Point) -> Point {
        self - 2.0 * self.dot(normal) * normal
    }

    /// Refract a point given an impact normal and index of refraction
    /// https://spie.org/publications/fg08_p13_index_of_refraction?SSO=1
    pub fn refract(self, normal: Point, refraction_index: f64) -> Point {
        // Calcualte parallel ray
        let cos_theta = (-1. * self).dot(normal).min(1.);
        let r_perpendicular = refraction_index * (self + cos_theta * normal);
        // Calcualte parallel ray
        let r_parallel = -1. * (1. - r_perpendicular.len().powi(2)).abs().sqrt() * normal;
        r_perpendicular + r_parallel
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add<Point> for f64 {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Sub<Point> for f64 {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self - rhs.x, self - rhs.y, self - rhs.z)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn can_create_default() {
        let v = Point::default();
        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
        assert_eq!(v.z, 0.);
    }

    #[test]
    fn can_create_random() {
        let v = Point::random(1.0..10.0);
        assert!(v.x >= 1.0 && v.x <= 10.0);
        assert!(v.y >= 1.0 && v.y <= 10.0);
        assert!(v.z >= 1.0 && v.z <= 10.0);
    }

    #[test]
    fn can_create_random_in_sphere() {
        let v = Point::random_in_sphere();
        assert!(v.x >= -1.0 && v.x <= 1.0);
        assert!(v.y >= -1.0 && v.y <= 1.0);
        assert!(v.z >= -1.0 && v.z <= 1.0);
        assert!(v.len() < 1.0)
    }

    #[test]
    fn can_add() {
        let v1 = Point::new(2., 2., 2.);
        let v2 = Point::new(1., 1., 1.);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 3.);
        assert_eq!(v3.y, 3.);
        assert_eq!(v3.z, 3.);
    }

    #[test]
    fn can_add_assign() {
        let mut v1 = Point::new(2., 2., 2.);
        let v2 = Point::new(1., 1., 1.);
        v1 += v2;
        assert_eq!(v1.x, 3.);
        assert_eq!(v1.y, 3.);
        assert_eq!(v1.z, 3.);
    }

    #[test]
    fn can_sub() {
        let v1 = Point::new(2., 2., 2.);
        let v2 = Point::new(1., 1., 1.);
        let v3 = v1 - v2;
        assert_eq!(v3.x, 1.);
        assert_eq!(v3.y, 1.);
        assert_eq!(v3.z, 1.);
    }

    #[test]
    fn can_sub_assign() {
        let mut v1 = Point::new(2., 2., 2.);
        let v2 = Point::new(1., 1., 1.);
        v1 -= v2;
        assert_eq!(v1.x, 1.);
        assert_eq!(v1.y, 1.);
        assert_eq!(v1.z, 1.);
    }

    #[test]
    fn can_mul() {
        let v1 = Point::new(2., 2., 2.);
        let v2 = 2.;
        let v3 = v1 * v2;
        assert_eq!(v3.x, 4.);
        assert_eq!(v3.y, 4.);
        assert_eq!(v3.z, 4.);
    }

    #[test]
    fn can_mul_assign() {
        let mut v1 = Point::new(2., 2., 2.);
        let v2 = 2.;
        v1 *= v2;
        assert_eq!(v1.x, 4.);
        assert_eq!(v1.y, 4.);
        assert_eq!(v1.z, 4.);
    }

    #[test]
    fn can_mul_assign_pt() {
        let v1 = 2.;
        let v2 = Point::new(3., 4., 5.);
        let v3 = v1 * v2;
        assert_eq!(v3.x, 6.);
        assert_eq!(v3.y, 8.);
        assert_eq!(v3.z, 10.);
    }

    #[test]
    fn can_div() {
        let v1 = Point::new(4., 4., 4.);
        let v2 = 2.;
        let v3 = v1 / v2;
        assert_eq!(v3.x, 2.);
        assert_eq!(v3.y, 2.);
        assert_eq!(v3.z, 2.);
    }

    #[test]
    fn can_div_assign() {
        let mut v1 = Point::new(4., 4., 4.);
        let v2 = 2.;
        v1 /= v2;
        assert_eq!(v1.x, 2.);
        assert_eq!(v1.y, 2.);
        assert_eq!(v1.z, 2.);
    }

    #[test]
    fn can_dot() {
        let v1 = Point::new(2., 2., 2.);
        let v2 = Point::new(3., 4., 5.);
        let v3 = v1.dot(v2);
        assert_eq!(v3, 6. + 8. + 10.);
    }

    #[test]
    fn can_len() {
        let v1 = Point::new(2., 2., 2.);
        let v2 = v1.len();
        println!("{v2}");
        assert!((v2 - 3.4641016151377544).abs() < f64::EPSILON)
    }

    #[test]
    fn can_cross() {
        let v1 = Point::new(2., 2., 2.);
        let v2 = Point::new(3., 4., 5.);
        let v3 = v1.cross(v2);
        assert_eq!(v3.x, 18.);
        assert_eq!(v3.y, -4.);
        assert_eq!(v3.z, 2.);
    }

    #[test]
    fn can_norm() {
        let v1 = Point::new(3., 2., -1.);
        let v2 = v1.normalized();
        assert!((v2.x - 0.8017837257372732).abs() < f64::EPSILON);
        assert!((v2.y - 0.5345224838248488).abs() < f64::EPSILON);
        assert!((v2.z + 0.2672612419124244).abs() < f64::EPSILON);
    }

    #[test]
    fn is_near_zero() {
        let v = Point::default();
        assert!(v.is_near_zero())
    }

    #[test]
    fn is_not_near_zero() {
        let v = Point::new(2., 2., 2.);
        assert!(!v.is_near_zero())
    }

    #[test]
    fn can_reflect() {
        let v = Point::new(2., 2., 2.);
        let n = Point::new(-1., 0., -1.);
        let reflected = v.reflect(n);
        assert!((reflected.x + 6.).abs() < f64::EPSILON);
        assert!((reflected.y - 2.).abs() < f64::EPSILON);
        assert!((reflected.z + 6.).abs() < f64::EPSILON);
    }

    #[test]
    fn can_refract() {
        let v = Point::new(2., 2., 2.);
        let n = Point::new(-1., 1., -1.);
        let refracted = v.refract(n, 1.0);
        assert!((refracted.x - 4.16227766016838).abs() < f64::EPSILON);
        assert!((refracted.y + 0.16227766016837952).abs() < f64::EPSILON);
        assert!((refracted.z - 4.16227766016838).abs() < f64::EPSILON);
    }
}
