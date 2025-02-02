
use crate::{
    colour::{Colour},
    lights::Light,
    math::tuple::Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    colour: Colour,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            colour: Colour::newi(1, 1, 1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: impl Light,
        point: Tuple,
        eye_vec: Tuple,
        normal_vec: Tuple,
    ) -> Colour {
        let diffuse: Colour;
        let specular: Colour;

        let effective_colour = self.colour * *light.intensity();
        let light_vec = (*light.position() - point).normalize();
        let ambient_light = effective_colour * self.ambient;

        let light_dot_normal = light_vec.dot(&normal_vec);
        if light_dot_normal < 0.0 {
            // Fast path, object (point) is between light and surface
            diffuse = Colour::BLACK;
            specular = Colour::BLACK;
        } else {
            diffuse = effective_colour * self.diffuse * light_dot_normal;
            let reflect_vec = (-light_vec).reflect(&normal_vec);
            let reflect_dot_eye = reflect_vec.dot(&eye_vec);
            specular = if reflect_dot_eye < 0.0 {
                Colour::BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                *light.intensity() * self.specular * factor
            }
        }

        ambient_light + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use crate::colour::Colour;

    use super::Material;

    #[test]
    fn construction_works() {
        let c: Material = Default::default();

        assert_eq!(c.ambient, 0.1);
        assert_eq!(c.specular, 0.9);
        assert_eq!(c.shininess, 200.0);
        assert_eq!(c.colour, Colour::newi(1, 1, 1))
    }

    mod lighting {
        use std::{default::Default, f64::consts::SQRT_2};

        use crate::{
            colour::Colour,
            lights::PointLight,
            materials::Material,
            math::tuple::{pointi, vectori, Tuple, ZERO_POINT},
        };

        #[test]
        fn eye_light_surface() {
            let (m, position): (Material, Tuple) = (Default::default(), ZERO_POINT);
            let eye_vec = Tuple::vectori(0, 0, -1);
            let normal_vec = vectori(0, 0, -1);
            let light = PointLight::new(Colour::newi(1, 1, 1), pointi(0, 0, -10));

            let res = m.lighting(light, position, eye_vec, normal_vec);
            assert_eq!(res, Colour::new(1.9, 1.9, 1.9))
        }
        #[test]
        fn eye_45deg_off_light_surface() {
            let (m, position): (Material, Tuple) = (Default::default(), ZERO_POINT);
            let eye_vec = Tuple::vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
            let normal_vec = vectori(0, 0, -1);
            let light = PointLight::new(Colour::newi(1, 1, 1), pointi(0, 0, -10));

            let res = m.lighting(light, position, eye_vec, normal_vec);
            assert_eq!(res, Colour::new(1.0, 1.0, 1.0))
        }

        #[test]
        fn light_45_degrees_off_surface() {
            let (m, position): (Material, Tuple) = (Default::default(), ZERO_POINT);
            let eye_vec = Tuple::vector(0.0, 0.0, -1.0);
            let normal_vec = vectori(0, 0, -1);
            let light = PointLight::new(Colour::newi(1, 1, 1), pointi(0, 10, -10));

            let res = m.lighting(light, position, eye_vec, normal_vec);
            assert_eq!(res, Colour::new(0.7364, 0.7364, 0.7364))
        }

        #[test]
        fn eye_direct_reflect_light_45deg() {
            let (m, position): (Material, Tuple) = (Default::default(), ZERO_POINT);
            let eye_vec = Tuple::vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
            let normal_vec = vectori(0, 0, -1);
            let light = PointLight::new(Colour::newi(1, 1, 1), pointi(0, 10, -10));

            let res = m.lighting(light, position, eye_vec, normal_vec);
            assert_eq!(res, Colour::new(1.6364, 1.6364, 1.6364))
        }

        #[test]
        fn eye_behind() {
            let (m, position): (Material, Tuple) = (Default::default(), ZERO_POINT);
            let eye_vec = Tuple::vector(0.0, 0.0, -1.0);
            let normal_vec = vectori(0, 0, -1);
            let light = PointLight::new(Colour::newi(1, 1, 1), pointi(0, 0, 10));

            let res = m.lighting(light, position, eye_vec, normal_vec);
            assert_eq!(res, Colour::new(0.1, 0.1, 0.1))
        }
    }
}
