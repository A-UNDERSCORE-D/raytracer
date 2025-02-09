use uuid::Uuid;

use crate::{
    materials::Material,
    math::{matrix::Matrix, tuple::Tuple},
    ray::RayIntersect,
};

pub mod sphere;

pub trait Shape: RayIntersect + std::fmt::Debug {
    fn id(&self) -> Uuid;
    // fn set_transform(&mut self, matrix: Matrix); // Because safety
    fn transform(&self) -> &Matrix;
    fn material(&self) -> &Material;
    fn normal_at(&self, point: Tuple) -> Tuple;
}

impl PartialEq for &dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
