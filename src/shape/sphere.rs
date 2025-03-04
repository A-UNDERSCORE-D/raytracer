use uuid::Uuid;

use crate::{
    intersection::Intersection,
    materials::Material,
    math::{
        matrix::Matrix,
        tuple::{Tuple, ZERO},
    },
    ray::Ray,
    shape::{shape_base, ShapeBase},
};

use super::Shape;

/// Its a sphere. What do you want from me?
#[derive(Debug, PartialEq)]
pub struct Sphere {
    _id: Uuid,
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new(transform: Matrix, material: Material) -> Self {
        //uuid
        Self {
            _id: Uuid::new_v4(),
            transform,
            material,
        }
    }

    pub fn new_with_transform(transform: Matrix) -> Self {
        Self::new(transform, Default::default())
    }

    pub fn new_with_material(material: Material) -> Self {
        Self::new(Default::default(), material)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

shape_base!(Sphere);

impl Shape for Sphere {
    fn local_normal_at(&self, point: Tuple) -> Tuple {
        point - ZERO // At any point, the vector for the normal is the exact opposite of the point (as a vec)
    }

    fn local_interception(&self, local_space_ray: Ray) -> Option<Vec<Intersection>> {
        let ray = local_space_ray;
        let s2r = ray.origin - Tuple::pointi(0, 0, 0);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&s2r);
        let c = s2r.dot(&s2r) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let disroot = discriminant.sqrt();
        Some(vec![
            Intersection::new((-b - disroot) / (2.0 * a), self),
            Intersection::new((-b + disroot) / (2.0 * a), self),
        ])
    }
}

#[cfg(test)]
mod test {

    use std::f64::consts::{PI, SQRT_2};

    use crate::{
        math::{
            matrix::{Matrix, IDENTITY_4X4},
            tuple::Tuple,
        },
        ray::{Ray, RayIntersect},
        shape::Shape,
    };

    use super::Sphere;

    // #[test]
    // fn set_transform() {
    //     let mut s = Sphere::default();
    //     assert_eq!(s.transform, IDENTITY_4X4.clone());
    //     s.set_transform(Matrix::translationi(1, 2, 3));

    //     assert_eq!(s.transform, Matrix::translationi(1, 2, 3));
    // }

    mod normal {
        use std::f64::consts::FRAC_1_SQRT_2;

        use super::*;
        macro_rules! normal_at {
            ($name:ident, $inp:expr, $out:expr) => {
                normal_at!($name, IDENTITY_4X4.clone(), $inp, $out);
            };
            ($name:ident, $transform:expr, $inp:expr, $out:expr) => {
                #[test]
                fn $name() {
                    let s = Sphere::new_with_transform($transform);
                    let n = s.normal_at($inp);

                    assert!(n.is_vector());
                    assert_eq!(n, $out);
                    assert_eq!(n, n.normalize());
                }
            };
        }

        normal_at!(
            normal_point_x,
            Tuple::pointi(1, 0, 0),
            Tuple::vectori(1, 0, 0)
        );

        normal_at!(
            normal_point_y,
            Tuple::pointi(0, 1, 0),
            Tuple::vectori(0, 1, 0)
        );

        normal_at!(
            normal_point_z,
            Tuple::pointi(0, 0, 1),
            Tuple::vectori(0, 0, 1)
        );

        normal_at!(
            normal_point_nonaxial,
            Tuple::point(
                (3.0_f64).sqrt() / 3.0,
                (3.0_f64).sqrt() / 3.0,
                (3.0_f64).sqrt() / 3.0
            ),
            Tuple::vector(
                (3.0_f64).sqrt() / 3.0,
                (3.0_f64).sqrt() / 3.0,
                (3.0_f64).sqrt() / 3.0
            )
        );

        normal_at!(
            translated,
            Matrix::translation(0.0, 1.0, 0.0),
            Tuple::point(0.0, 1.70711, -FRAC_1_SQRT_2),
            Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );
        normal_at!(
            transformed,
            Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0),
            Tuple::point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0),
            Tuple::vector(0.0, 0.97014, -0.24254)
        );
    }

    mod intersect {
        use super::*;
        #[test]
        fn two_points() {
            let r = Ray::new(Tuple::pointi(0, 0, -5), Tuple::vectori(0, 0, 1));
            let s = Sphere::default();

            let xs = s.intersect(r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, 4.0);
            assert_eq!(xs[1].t, 6.0);
        }

        #[test]
        fn tangent() {
            let r = Ray::new(Tuple::pointi(0, 1, -5), Tuple::vectori(0, 0, 1));
            let s = Sphere::default();

            let xs = s.intersect(r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, 5.0);
            assert_eq!(xs[1].t, 5.0);
        }

        #[test]
        fn none() {
            let r = Ray::new(Tuple::pointi(0, 2, -5), Tuple::vectori(0, 0, 1));
            let s = Sphere::default();

            let xs = s.intersect(r);
            assert!(xs.is_none());
        }

        #[test]
        fn center() {
            let r = Ray::new(Tuple::pointi(0, 0, 0), Tuple::vectori(0, 0, 1));
            let s = Sphere::default();

            let xs = s.intersect(r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, -1.0);
            assert_eq!(xs[1].t, 1.0);
        }

        #[test]
        fn behind() {
            let r = Ray::new(Tuple::pointi(0, 0, 5), Tuple::vectori(0, 0, 1));
            let s = Sphere::default();

            let xs = s.intersect(r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, -6.0);
            assert_eq!(xs[1].t, -4.0);
        }

        #[test]
        fn scaled() {
            let r = Ray::new(Tuple::pointi(0, 0, -5), Tuple::vectori(0, 0, 1));
            let s = Sphere::new_with_transform(Matrix::scalingi(2, 2, 2));

            let xs = s.intersect(r).expect("Did not get expected intersections");

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, 3.0);
            assert_eq!(xs[1].t, 7.0);
        }
        #[test]
        fn translated() {
            let r = Ray::new(Tuple::pointi(0, 0, -5), Tuple::vectori(0, 0, 1));
            let s: Sphere = Sphere::new_with_transform(Matrix::translationi(5, 0, 0));

            let xs: Option<Vec<crate::intersection::Intersection<'_>>> = s.intersect(r);
            assert!(xs.is_none())
        }
    }
}
