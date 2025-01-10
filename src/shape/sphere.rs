use uuid::Uuid;

use crate::{
    math::{
        matrix::{Matrix, IDENTITY_4X4},
        tuple::Tuple,
    },
    ray::{Intersection, Ray, RayIntersect},
};

use super::Shape;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    _id: Uuid,
    pub transform: Matrix,
}

#[allow(clippy::new_without_default)]
impl Sphere {
    pub fn new() -> Self {
        //uuid
        Self::new_with_transform(IDENTITY_4X4.clone())
    }

    pub fn new_with_transform(transform: Matrix) -> Self {
        Self {
            _id: Uuid::new_v4(),
            transform,
        }
    }
}

impl RayIntersect for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let ray = ray.transform(
            &self
                .transform
                .inverse()
                .expect("Must be able to invert the translation of the shape"),
        );
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

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        self._id
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform
    }
}

#[cfg(test)]
mod test {

    use crate::{
        math::{matrix::Matrix, matrix::IDENTITY_4X4, tuple::Tuple},
        ray::{Ray, RayIntersect},
        shape::Shape,
    };

    use super::Sphere;

    #[test]
    fn set_transform() {
        let mut s = Sphere::new();
        assert_eq!(s.transform, IDENTITY_4X4.clone());
        s.set_transform(Matrix::translationi(1, 2, 3));

        assert_eq!(s.transform, Matrix::translationi(1, 2, 3));
    }

    mod intersect {
        use super::*;
        #[test]
        fn two_points() {
            let r = Ray::new(Tuple::pointi(0, 0, -5), Tuple::vectori(0, 0, 1));
            let s = Sphere::new();

            let xs = s.intersect(r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, 4.0);
            assert_eq!(xs[1].t, 6.0);
        }

        #[test]
        fn tangent() {
            let r = Ray::new(Tuple::pointi(0, 1, -5), Tuple::vectori(0, 0, 1));
            let s = Sphere::new();

            let xs = s.intersect(r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, 5.0);
            assert_eq!(xs[1].t, 5.0);
        }

        #[test]
        fn none() {
            let r = Ray::new(Tuple::pointi(0, 2, -5), Tuple::vectori(0, 0, 1));
            let s = Sphere::new();

            let xs = s.intersect(r);
            assert!(xs.is_none());
        }

        #[test]
        fn center() {
            let r = Ray::new(Tuple::pointi(0, 0, 0), Tuple::vectori(0, 0, 1));
            let s = Sphere::new();

            let xs = s.intersect(r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, -1.0);
            assert_eq!(xs[1].t, 1.0);
        }

        #[test]
        fn behind() {
            let r = Ray::new(Tuple::pointi(0, 0, 5), Tuple::vectori(0, 0, 1));
            let s = Sphere::new();

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
            let s = Sphere::new_with_transform(Matrix::translationi(5, 0, 0));

            let xs = s.intersect(r);
            assert!(xs.is_none())
        }
    }
}
