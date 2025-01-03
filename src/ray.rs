
use crate::math::tuple::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        assert!(origin.is_point());
        assert!(direction.is_vector());

        Self { origin, direction }
    }
}

impl Ray {
    pub fn position(&self, dst: f64) -> Tuple {
        self.origin + (self.direction * dst)
    }
}

pub trait RayIntersect {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>>;
}

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn RayIntersect,
}

impl<'a> Intersection<'a> {
    pub fn new<T: RayIntersect + 'a>(t: f64, object: &'a T) -> Self {
        Self { t, object }
    }
}

#[cfg(test)]
mod test {
    use crate::math::tuple::Tuple;

    use super::Ray;

    #[test]
    fn position() {
        let r = Ray::new(Tuple::pointi(2, 3, 4), Tuple::vectori(1, 0, 0));

        assert_eq!(r.position(0.0), r.origin);
        assert_eq!(r.position(1.0), Tuple::pointi(3, 3, 4));
        assert_eq!(r.position(1.0), Tuple::pointi(3, 3, 4));
        assert_eq!(r.position(-1.0), Tuple::pointi(1, 3, 4));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }
}
