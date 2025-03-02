use std::{
    cell::RefCell,
    sync::Mutex,
};

use crate::{
    materials::Material,
    math::{matrix::Matrix, tuple::Tuple},
    ray::Ray,
};

use super::Shape;

/// Not really used anywhere, mostly used o verify implementations (if any) in Shape
#[derive(Debug, Default)]
pub struct TestShape {
    id: uuid::Uuid,
    pub transform: Matrix,
    pub material: Material,

    saved_ray: Mutex<RefCell<Ray>>,
}

impl Shape for TestShape {
    fn id(&self) -> uuid::Uuid {
        self.id
    }
    fn material(&self) -> &Material {
        &self.material
    }
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        Tuple::vector(point.x, point.y, point.z)
    }

    fn local_interception(
        &self,
        local_space_ray: Ray,
    ) -> Option<Vec<crate::intersection::Intersection>> {
        self.saved_ray.lock().unwrap().replace(local_space_ray);

        None
    }

    fn set_material(&mut self, material: Material) {
        self.material = material
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform
    }
}

#[cfg(test)]
mod test {
    // I think this is silly but...

    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use crate::{
        math::{
            matrix::{Matrix, IDENTITY_4X4},
            tuple::{point, pointi, vector, vectori},
        },
        ray::{Ray, RayIntersect},
        shape::Shape,
    };

    use super::TestShape;

    #[test]
    fn verify_construct() {
        let s: &dyn Shape = &TestShape::default();
        assert_eq!(s.transform(), &*IDENTITY_4X4);
    }

    #[test]
    fn verify_mut() {
        let s: &mut dyn Shape = &mut TestShape::default();
        s.set_transform(Matrix::translationi(2, 3, 4));

        assert_eq!(s.transform(), &Matrix::translationi(2, 3, 4));
    }

    #[test]
    fn ray_intersect_scaled() {
        let r = Ray::new(pointi(0, 0, -5), vectori(0, 0, 1));
        let mut s = TestShape::default();
        s.set_transform(Matrix::scalingi(2, 2, 2));

        s.intersect(r);
        let saved = *s.saved_ray.lock().unwrap().borrow();

        assert_eq!(saved.origin, point(0.0, 0.0, -2.5));
        assert_eq!(saved.direction, vector(0.0, 0.0, 0.5))
    }
    #[test]
    fn ray_intersect_translated() {
        let r = Ray::new(pointi(0, 0, -5), vectori(0, 0, 1));
        let mut s = TestShape::default();
        s.set_transform(Matrix::translationi(5, 0, 0));

        s.intersect(r);
        let saved = *s.saved_ray.lock().unwrap().borrow();

        assert_eq!(saved.origin, pointi(-5, 0, -5));
        assert_eq!(saved.direction, vectori(0, 0, 1))
    }

    #[test]
    fn normal_translated() {
        let mut s = TestShape::default();
        s.set_transform(Matrix::translationi(0, 1, 0));

        let normal = s.normal_at(point(0.0, 1.70711, -FRAC_1_SQRT_2));

        assert_eq!(normal, vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2))
    }

    #[test]
    fn normal_transformed() {
        let mut s = TestShape::default();
        s.set_transform(Matrix::rotation_z(PI / 5.0).scale(1.0, 0.5, 1.0));

        let normal = s.normal_at(point(
            0.0,
            (2.0_f64.sqrt()) / 2.0,
            -((2.0_f64.sqrt()) / 2.0),
        ));

        assert_eq!(normal, vector(0.0, 0.97014, -0.24254))
    }
}
