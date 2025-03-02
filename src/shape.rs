use uuid::Uuid;

use crate::{
    intersection::Intersection,
    materials::Material,
    math::{matrix::Matrix, tuple::Tuple},
    ray::{Ray, RayIntersect},
};

pub mod sphere;
mod test_shape;

pub trait Shape: std::fmt::Debug {
    fn id(&self) -> Uuid;
    fn transform(&self) -> &Matrix;
    fn material(&self) -> &Material;
    fn local_interception(&self, local_space_ray: Ray) -> Option<Vec<Intersection>>;
    fn set_transform(&mut self, transform: Matrix);
    fn set_material(&mut self, material: Material);
    fn local_normal_at(&self, point: Tuple) -> Tuple;
    fn normal_at(&self, point: Tuple) -> Tuple {
        let inverted = &self.transform().inverse().unwrap();
        let local_point = inverted * point;
        let local_normal = self.local_normal_at(local_point);

        let mut world_point = inverted.transpose() * local_normal;
        world_point.w = 0.0;

        world_point.normalize()
    }
}

impl<T: ?Sized> RayIntersect for T
where
    T: Shape,
{
    fn intersect(&self, ray: crate::ray::Ray) -> Option<Vec<crate::intersection::Intersection>> {
        let local_ray = ray.transform(
            &self
                .transform()
                .inverse()
                .expect("transform must be invertable"),
        );
        self.local_interception(local_ray)
    }
}

impl PartialEq for &dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
