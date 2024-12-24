use std::ops::{Add, Mul, Neg, Sub};

use super::{float::equal, matrix};

pub const ZERO: Tuple = Tuple {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

/// Constructors
impl Tuple {
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Self { x, y, z, w: 0.0 }
    }
    pub fn vectori(x: i32, y: i32, z: i32) -> Tuple {
        Self::vector(x as f64, y as f64, z as f64)
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Self { x, y, z, w: 1.0 }
    }
    pub fn pointi(x: i32, y: i32, z: i32) -> Tuple {
        Self::point(x as f64, y as f64, z as f64)
    }
}

/// actual methods
impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Self) -> Tuple {
        if !self.is_vector() || !other.is_vector() {
            panic!("cross product of non-vectors not supported")
        }

        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
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

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<u32> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: u32) -> Self::Output {
        self * rhs as f64
    }
}

impl From<matrix::Ref<'_>> for Tuple {
    fn from(value: matrix::Ref) -> Self {
        Tuple {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
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

    #[test]
    fn test_neg() {
        assert_eq!(
            -Tuple::vector(1.0, -2.0, 3.0),
            Tuple::vector(-1.0, 2.0, -3.0)
        )
    }

    #[test]
    fn test_mul_scalar() {
        assert_eq!(
            Tuple::vector(1.0, 2.0, 3.0) * 2,
            Tuple::vector(2.0, 4.0, 6.0)
        )
    }

    #[test]
    fn test_mul_scalar_float() {
        assert_eq!(
            Tuple {
                x: 1.0,
                y: -2.0,
                z: 3.0,
                w: -4.0
            } * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        )
    }

    mod magnitude_tests {
        use super::*;
        macro_rules! mag_test {
            ($name:ident, $input:expr, $expected:expr) => {
                #[test]
                fn $name() {
                    assert_eq!(($input).magnitude(), $expected as f64)
                }
            };
        }

        mag_test!(unit_vector_1, Tuple::vector(1.0, 0.0, 0.0), 1);
        mag_test!(unit_vector_2, Tuple::vector(0.0, 1.0, 0.0), 1);
        mag_test!(unit_vector_3, Tuple::vector(0.0, 0.0, 1.0), 1);
        mag_test!(magnitude, Tuple::vector(1.0, 2.0, 3.0), 14.0_f64.sqrt());
        mag_test!(ozz, Tuple::vector(-1.0, -2.0, -3.0), 14.0_f64.sqrt());
    }

    mod normal_tests {
        use super::*;
        macro_rules! normalize_test {
            ($name:ident, $input:expr, $expected:expr) => {
                #[test]
                fn $name() {
                    assert_eq!($input.normalize(), $expected)
                }
            };
        }

        normalize_test!(four, Tuple::vector(3.0, 0.0, 0.0), Tuple::vectori(1, 0, 0));
        normalize_test!(
            complex,
            Tuple::vectori(1, 2, 3),
            Tuple::vector(
                1.0 / 14_f64.sqrt(),
                2.0 / 14_f64.sqrt(),
                3.0 / 14_f64.sqrt()
            )
        );

        #[test]
        fn verify_magnitude() {
            let vec = Tuple::vectori(1, 2, 3);
            assert_eq!(vec.normalize().magnitude(), 1.0)
        }
    }

    #[test]
    fn dot() {
        let a = Tuple::vectori(1, 2, 3);
        let b = Tuple::vectori(2, 3, 4);

        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross() {
        let a = Tuple::vectori(1, 2, 3);
        let b = Tuple::vectori(2, 3, 4);

        assert_eq!(Tuple::cross(&a, &b), Tuple::vectori(-1, 2, -1));
        assert_eq!(Tuple::cross(&b, &a), Tuple::vectori(1, -2, 1))
    }
}
