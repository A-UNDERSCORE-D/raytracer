use core::panicking::panic_const::panic_const_async_gen_fn_resumed_panic;
use std::ops::{Add, Div, Mul, Sub};

use crate::math::float::equal;

#[derive(Clone, Copy, Debug, Default)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

macro_rules! impl_trait_simple {
    ($trait:ident, $funcname:ident, $op:tt) => {
        impl $trait for Colour {
            type Output = Colour;
            fn $funcname(self, rhs: Self) -> Self::Output {
                Self {
                    red: self.red $op rhs.red,
                    green: self.green $op rhs.green,
                    blue: self.blue $op rhs.blue,
                }
            }
        }
    };
}

impl_trait_simple!(Add, add, +);
impl_trait_simple!(Sub, sub, -);
impl_trait_simple!(Mul, mul, *);
impl_trait_simple!(Div, div, /);

impl Colour {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub const fn newi(red: i32, green: i32, blue: i32) -> Self {
        Self::new(red as f64, green as f64, blue as f64)
    }

    pub fn to_ppm(&self) -> String {
        const MAX_NUM: f64 = 255.0;
        format!(
            "{} {} {}",
            (self.red * MAX_NUM).round().clamp(0.0, MAX_NUM) as u64,
            (self.green * MAX_NUM).round().clamp(0.0, MAX_NUM) as u64,
            (self.blue * MAX_NUM).round().clamp(0.0, MAX_NUM) as u64,
        )
    }
}

impl Mul<f64> for Colour {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}
impl Mul<i32> for Colour {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        self * rhs as f64
    }
}
impl PartialEq for Colour {
    fn eq(&self, other: &Self) -> bool {
        equal(self.red, other.red) && equal(self.green, other.green) && equal(self.blue, other.blue)
    }
}

/// Namespaced colour defaults for ease of use
impl Colour {
    pub const RED: Colour = Colour::newi(255, 0, 0);
    pub const BLACK: Colour = Colour::newi(0, 0, 0);
}

#[cfg(test)]
mod test {
    use super::Colour;

    #[test]
    fn colours_work() {
        let c = Colour {
            red: 1.0,
            green: 2.0,
            blue: 3.0,
        };

        assert_eq!(c.red, 1.0);
        assert_eq!(c.green, 2.0);
        assert_eq!(c.blue, 3.0);
    }

    #[test]
    fn simple_ops() {
        let left = Colour::new(0.9, 0.6, 0.75);
        let right = Colour::new(0.7, 0.1, 0.25);

        assert_eq!(left + right, Colour::new(1.6, 0.7, 1.0));
        assert_eq!(left - right, Colour::new(0.2, 00.5, 0.5));
    }

    #[test]
    fn mul_scalar() {
        assert_eq!(Colour::new(0.2, 0.3, 0.4) * 2, Colour::new(0.4, 0.6, 0.8));
    }

    mod ppm {
        use crate::colour::Colour;

        #[test]
        fn simple() {
            assert_eq!(Colour::default().to_ppm(), "0 0 0")
        }

        #[test]
        fn clamped() {
            let c = Colour::new(-1.0, 0.5, 2.0);
            assert_eq!(c.to_ppm(), "0 128 255")
        }
    }
}
