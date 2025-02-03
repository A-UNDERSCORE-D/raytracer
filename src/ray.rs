use crate::{
    intersection::Intersection,
    math::{matrix::Matrix, tuple::Tuple},
};

#[derive(Debug, Clone, Copy)]
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

    pub fn transform(&self, matrix: &Matrix) -> Self {
        Self::new(matrix * self.origin, matrix * self.direction)
    }
}

// Used by shape
pub trait RayIntersect {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>>;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn position() {
        let r = Ray::new(Tuple::pointi(2, 3, 4), Tuple::vectori(1, 0, 0));

        assert_eq!(r.position(0.0), r.origin);
        assert_eq!(r.position(1.0), Tuple::pointi(3, 3, 4));
        assert_eq!(r.position(1.0), Tuple::pointi(3, 3, 4));
        assert_eq!(r.position(-1.0), Tuple::pointi(1, 3, 4));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate() {
        let r = Ray::new(Tuple::pointi(1, 2, 3), Tuple::vectori(0, 1, 0));
        let m = &Matrix::translationi(3, 4, 5);

        let res = r.transform(m);

        assert_eq!(res.origin, Tuple::pointi(4, 6, 8));
        assert_eq!(res.direction, Tuple::vectori(0, 1, 0));
    }
    #[test]
    fn scale() {
        let r = Ray::new(Tuple::pointi(1, 2, 3), Tuple::vectori(0, 1, 0));
        let m = &Matrix::scalingi(2, 3, 4);

        let res = r.transform(m);

        assert_eq!(res.origin, Tuple::pointi(2, 6, 12));
        assert_eq!(res.direction, Tuple::vectori(0, 3, 0));
    }
}
