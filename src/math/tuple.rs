use std::ops::{Add, Neg, Sub};

use super::float::equal;

pub const ZERO: Tuple = Tuple {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};

#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Self { x, y, z, w: 0.0 }
    }
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Self { x, y, z, w: 1.0 }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w)
    }
}
impl Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;
    #[test]
    fn point() {
        // Dumb test, but Im following el book and its a nice just in case thing

        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert!(t.x == 4.3);
        assert!(t.y == -4.2);
        assert!(t.z == 3.1);

        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn test_eq() {
        assert!(Tuple::point(1.0, 2.0, 3.0) == Tuple::point(1.0, 2.0, 3.0))
    }

    #[test]
    fn test_add() {
        let a = Tuple::vector(3.0, -2.0, 5.0);
        let b = Tuple::point(-2.0, 3.0, 1.0);

        let expected = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };

        assert!(a + b == expected)
    }

    #[test]
    /// Subtract points from eachother getting the vector between them
    fn test_sub() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::point(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        let res = a - b;

        assert!(res == expected, "want {:?}, got {:?}", expected, res)
    }

    #[test]
    fn test_sub_vec_point() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::point(-2.0, -4.0, -6.0);

        let res = a - b;

        assert!(res == expected, "want {:?}, got {:?}", expected, res)
    }

    #[test]
    fn test_sub_vec_vec() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        let res = a - b;

        assert!(res == expected, "want {:?}, got {:?}", expected, res)
    }

    #[test]
    fn vector() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert!(t.x == 4.3);
        assert!(t.y == -4.2);
        assert!(t.z == 3.1);

        assert!(!t.is_point());
        assert!(t.is_vector());
    }
}
