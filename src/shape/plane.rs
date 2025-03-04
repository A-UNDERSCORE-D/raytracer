// Is it a bird? No, no it is not

use uuid::Uuid;

use crate::{
    intersection::Intersection,
    materials::Material,
    math::{float::EPSILON, matrix::Matrix, tuple::vectori},
};

use super::{shape_base, Shape, ShapeBase};

#[derive(Debug, Clone)]
pub struct Plane {
    _id: uuid::Uuid,
    pub transform: Matrix,
    pub material: Material,
}

shape_base!(Plane);
impl Plane {
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

impl Default for Plane {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl Shape for Plane {
    fn local_interception(
        &self,
        local_space_ray: crate::ray::Ray,
    ) -> Option<Vec<crate::intersection::Intersection>> {
        if local_space_ray.direction.y.abs() < EPSILON {
            None
        } else {
            let t = -local_space_ray.origin.y / local_space_ray.direction.y;
            Some(vec![Intersection::new(t, self)])
        }
    }
    #[inline]
    fn local_normal_at(&self, _: crate::math::tuple::Tuple) -> crate::math::tuple::Tuple {
        vectori(0, 1, 0)
    }
}

#[cfg(test)]
mod test {

    use crate::{
        math::tuple::{pointi, vectori},
        ray::Ray,
        shape::Shape,
    };

    use super::Plane;

    #[test]
    fn local_normal_at() {
        let p: Plane = Default::default();

        let n1 = p.local_normal_at(pointi(0, 0, 0));
        let n2 = p.local_normal_at(pointi(10, 0, -10));
        let n3 = p.local_normal_at(pointi(-5, 0, 150));

        for n in [n1, n2, n3] {
            assert_eq!(n, vectori(0, 1, 0))
        }
    }

    #[test]
    fn intercept_parallel() {
        let p = Plane::default();

        let r = Ray::new(pointi(0, 10, 0), vectori(0, 0, 1));

        let xs = p.local_interception(r);
        assert!(xs.is_none())
    }

    #[test]
    fn intercept_coplanar() {
        let p = Plane::default();

        let r = Ray::new(pointi(0, 0, 0), vectori(0, 0, 1));

        let xs = p.local_interception(r);
        assert!(xs.is_none())
    }

    #[test]
    fn intercept_above() {
        let p = Plane::default();

        let r = Ray::new(pointi(0, 1, 0), vectori(0, -1, 0));

        let xs = p.local_interception(r).unwrap();
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p)
    }

    #[test]
    fn intercept_below() {
        let p = Plane::default();

        let r = Ray::new(pointi(0, -1, 0), vectori(0, 1, 0));

        let xs = p.local_interception(r).unwrap();
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p)
    }
}
