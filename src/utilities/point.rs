use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
    pub fn dot(self, rhs: Point) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// A vector dotted with itself is equal to the squared length of that vector
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }

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
}
