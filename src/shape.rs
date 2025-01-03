use crate::ray::RayIntersect;

pub mod sphere;

pub trait Shape: RayIntersect {}
