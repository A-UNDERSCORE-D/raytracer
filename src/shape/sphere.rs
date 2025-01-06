use uuid::Uuid;

use crate::{
    math::tuple::Tuple,
    ray::{Intersection, Ray, RayIntersect},
};

use super::Shape;

#[derive(Debug, PartialEq, Eq)]
pub struct Sphere {
    _id: Uuid,
}

#[allow(clippy::new_without_default)] // more stuff soon (tm)
impl Sphere {
    pub fn new() -> Self {
        //uuid
        Sphere {
            _id: Uuid::new_v4(),
        }
    }
}

impl RayIntersect for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
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
}

#[cfg(test)]
mod test {

    use crate::{
        math::tuple::Tuple,
        ray::{Ray, RayIntersect},
    };

    use super::Sphere;

    #[test]
    fn intersect_two_points() {
        let r = Ray::new(Tuple::pointi(0, 0, -5), Tuple::vectori(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r).unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn intersect_tangent() {
        let r = Ray::new(Tuple::pointi(0, 1, -5), Tuple::vectori(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r).unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn intersect_none() {
        let r = Ray::new(Tuple::pointi(0, 2, -5), Tuple::vectori(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r);
        assert!(xs.is_none());
    }

    #[test]
    fn intersect_center() {
        let r = Ray::new(Tuple::pointi(0, 0, 0), Tuple::vectori(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r).unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn intersect_behind() {
        let r = Ray::new(Tuple::pointi(0, 0, 5), Tuple::vectori(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r).unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
}
