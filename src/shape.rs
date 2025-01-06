use uuid::Uuid;

use crate::ray::RayIntersect;

pub mod sphere;

pub trait Shape: RayIntersect + std::fmt::Debug {
    fn id(&self) -> Uuid;
}

impl PartialEq for &dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
