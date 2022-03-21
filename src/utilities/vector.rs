use std::default;

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Default for Vector {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn can_create_default() {
        let v = Vector::default();
        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
        assert_eq!(v.z, 0.);
    }
}
