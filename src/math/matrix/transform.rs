use super::Matrix;
use super::IDENTITY_4X4;
impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut base = IDENTITY_4X4.clone();
        base[(0, 3)] = x;
        base[(1, 3)] = y;
        base[(2, 3)] = z;
        base
    }

    pub fn translationi(x: i32, y: i32, z: i32) -> Self {
        Self::translation(x as f64, y as f64, z as f64)
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut base = IDENTITY_4X4.clone();
        base[(0, 0)] = x;
        base[(1, 1)] = y;
        base[(2, 2)] = z;
        base
    }

    pub fn scalingi(x: i32, y: i32, z: i32) -> Self {
        Self::scaling(x as f64, y as f64, z as f64)
    }

    pub fn rotation_x(radians: f64) -> Self {
        let mut out = IDENTITY_4X4.clone();

        let sin = radians.sin();
        let cos = radians.cos();

        out[(1, 1)] = cos;
        out[(1, 2)] = -sin;
        out[(2, 1)] = sin;
        out[(2, 2)] = cos;

        out
    }

    pub fn rotation_y(radians: f64) -> Self {
        let mut out = IDENTITY_4X4.clone();

        let sin = radians.sin();
        let cos = radians.cos();

        out[(0, 0)] = cos;
        out[(0, 2)] = sin;
        out[(2, 0)] = -sin;
        out[(2, 2)] = cos;

        out
    }

    pub fn rotatation_z(radians: f64) -> Self {
        let mut out = IDENTITY_4X4.clone();

        let sin = radians.sin();
        let cos = radians.cos();

        out[(0, 0)] = cos;
        out[(0, 1)] = -sin;
        out[(1, 0)] = sin;
        out[(1, 1)] = cos;

        out
    }

    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self {
        let mut out = IDENTITY_4X4.clone();

        out[(0, 1)] = x_y;
        out[(0, 2)] = x_z;
        out[(1, 0)] = y_x;
        out[(1, 2)] = y_z;
        out[(2, 0)] = z_x;
        out[(2, 1)] = z_y;

        out
    }
    pub fn shearingi(x_y: i32, x_z: i32, y_x: i32, y_z: i32, z_x: i32, z_y: i32) -> Self {
        Self::shearing(
            x_y as f64, x_z as f64, y_x as f64, y_z as f64, z_x as f64, z_y as f64,
        )
    }
}

impl Matrix {
    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Self::translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Self::scaling(x, y, z) * self
    }
    pub fn rotate_x(self, radians: f64) -> Self {
        Self::rotation_x(radians) * self
    }

    pub fn rotate_y(self, radians: f64) -> Self {
        Self::rotation_y(radians) * self
    }

    pub fn rotate_z(self, radians: f64) -> Self {
        Self::rotatation_z(radians) * self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::tuple::Tuple;
    use std::f64::consts::FRAC_PI_2;

    macro_rules! translation_test {
        ($name:ident, $matrix:expr, $tuple:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let transform: Matrix = $matrix;
                let tuple: Tuple = $tuple;

                assert_eq!(transform * tuple, $expected);
            }
        };
    }

    translation_test!(
        trans_simple,
        Matrix::translationi(5, -3, 2),
        Tuple::pointi(-3, 4, 5),
        Tuple::pointi(2, 1, 7)
    );

    translation_test!(
        trans_inverse,
        Matrix::translationi(5, -3, 2).inverse().unwrap(),
        Tuple::pointi(-3, 4, 5),
        Tuple::pointi(-8, 7, 3)
    );

    translation_test!(
        scale_simple,
        Matrix::scalingi(2, 3, 4),
        Tuple::pointi(-4, 6, 8),
        Tuple::pointi(-8, 18, 32)
    );

    translation_test!(
        scale_vec,
        Matrix::scalingi(2, 3, 4),
        Tuple::vectori(-4, 6, 8),
        Tuple::vectori(-8, 18, 32)
    );

    translation_test!(
        scale_reflect,
        Matrix::scalingi(-1, 1, 1),
        Tuple::pointi(2, 3, 4),
        Tuple::pointi(-2, 3, 4)
    );

    translation_test!(
        rotate_x_half_quarter,
        Matrix::rotation_x(45_f64.to_radians()),
        Tuple::pointi(0, 1, 0),
        Tuple::point(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
    );
    translation_test!(
        rotate_x_quarter,
        Matrix::rotation_x(90_f64.to_radians()),
        Tuple::pointi(0, 1, 0),
        Tuple::pointi(0, 0, 1)
    );

    translation_test!(
        rotate_x_half_quarter_inverse,
        Matrix::rotation_x(45_f64.to_radians()).inverse().unwrap(),
        Tuple::pointi(0, 1, 0),
        Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt() / 2.0))
    );

    translation_test!(
        rotate_y_half_quater,
        Matrix::rotation_y(std::f64::consts::FRAC_PI_4),
        Tuple::pointi(0, 0, 1),
        Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
    );
    translation_test!(
        rotate_y_quater,
        Matrix::rotation_y(std::f64::consts::FRAC_PI_2),
        Tuple::pointi(0, 0, 1),
        Tuple::pointi(1, 0, 0)
    );
    translation_test!(
        rotate_z_half_quater,
        Matrix::rotatation_z(std::f64::consts::FRAC_PI_4),
        Tuple::pointi(0, 1, 0),
        Tuple::point(-(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0, 0.0)
    );
    translation_test!(
        rotate_z_quater,
        Matrix::rotatation_z(std::f64::consts::FRAC_PI_2),
        Tuple::pointi(0, 1, 0),
        Tuple::pointi(-1, 0, 0)
    );

    translation_test!(
        sheari_x_y,
        Matrix::shearingi(1, 0, 0, 0, 0, 0),
        Tuple::pointi(2, 3, 4),
        Tuple::pointi(5, 3, 4)
    );
    translation_test!(
        sheari_x_z,
        Matrix::shearingi(0, 1, 0, 0, 0, 0),
        Tuple::pointi(2, 3, 4),
        Tuple::pointi(6, 3, 4)
    );
    translation_test!(
        sheari_y_x,
        Matrix::shearingi(0, 0, 1, 0, 0, 0),
        Tuple::pointi(2, 3, 4),
        Tuple::pointi(2, 5, 4)
    );
    translation_test!(
        sheari_y_z,
        Matrix::shearingi(0, 0, 0, 1, 0, 0),
        Tuple::pointi(2, 3, 4),
        Tuple::pointi(2, 7, 4)
    );
    translation_test!(
        sheari_z_x,
        Matrix::shearingi(0, 0, 0, 0, 1, 0),
        Tuple::pointi(2, 3, 4),
        Tuple::pointi(2, 3, 6)
    );
    translation_test!(
        sheari_z_y,
        Matrix::shearingi(0, 0, 0, 0, 0, 1),
        Tuple::pointi(2, 3, 4),
        Tuple::pointi(2, 3, 7)
    );

    #[test]
    fn chained_transforms() {
        let start = Tuple::pointi(1, 0, 1);

        let rotx = &Matrix::rotation_x(FRAC_PI_2);
        let scale = &Matrix::scalingi(5, 5, 5);
        let trans = &Matrix::translationi(10, 5, 7);

        let rotated = rotx * start;
        let scaled = scale * rotated;
        let translated = trans * scaled;

        assert_eq!(rotated, Tuple::pointi(1, -1, 0), "rotation not as expected");
        assert_eq!(scaled, Tuple::pointi(5, -5, 0), "scale not as expected");
        assert_eq!(
            translated,
            Tuple::pointi(15, 0, 7),
            "translation not as expected"
        );

        let chained = trans * scale * rotx;
        assert_eq!(
            chained * start,
            translated,
            "Chained did not result in the same result"
        );

        let fluent = IDENTITY_4X4
            .clone()
            .rotate_x(FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        assert_eq!(fluent * start, translated);
    }
}
