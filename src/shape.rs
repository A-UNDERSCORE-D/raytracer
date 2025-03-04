use uuid::Uuid;

use crate::{
    intersection::Intersection,
    materials::Material,
    math::{matrix::Matrix, tuple::Tuple},
    ray::{Ray, RayIntersect},
};

pub mod plane;
pub mod sphere;
mod test_shape;

pub trait ShapeBase {
    fn id(&self) -> Uuid;
    fn transform(&self) -> &Matrix;
    fn material(&self) -> &Material;
    fn set_transform(&mut self, transform: Matrix);
    fn set_material(&mut self, material: Material);
}

pub trait Shape: std::fmt::Debug + ShapeBase {
    fn local_interception(&self, local_space_ray: Ray) -> Option<Vec<Intersection>>;
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

macro_rules! shape_base {
    ($name:ident) => {
        impl ShapeBase for $name {
            fn id(&self) -> uuid::Uuid {
                self._id
            }
            fn material(&self) -> &Material {
                &self.material
            }
            fn transform(&self) -> &Matrix {
                &self.transform
            }

            fn set_material(&mut self, material: Material) {
                self.material = material
            }

            fn set_transform(&mut self, transform: Matrix) {
                self.transform = transform
            }
        }
    };
}

pub(crate) use shape_base; // Reexport this to make it make more sense
