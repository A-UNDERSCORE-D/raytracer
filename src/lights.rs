use std::fmt::Debug;

use crate::{colour::Colour, math::tuple::Tuple};

pub trait Light: Debug {
    fn intensity(&self) -> &Colour;
    fn position(&self) -> &Tuple;
}

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub intensity: Colour,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Colour, position: Tuple) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}

impl Light for PointLight {
    fn intensity(&self) -> &Colour {
        &self.intensity
    }

    fn position(&self) -> &Tuple {
        &self.position
    }
}

#[cfg(test)]
mod tests {
    use crate::{colour::Colour, math::tuple::ZERO};

    use super::PointLight;

    #[test]
    fn construction_works() {
        let l = PointLight {
            intensity: Colour::BLACK,
            position: ZERO,
        };

        assert_eq!(l.intensity, Colour::BLACK);
        assert_eq!(l.position, ZERO)
    }
}
