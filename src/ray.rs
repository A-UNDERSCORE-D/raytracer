use crate::{
    math::{matrix::Matrix, tuple::Tuple},
    shape::Shape,
};

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

#[derive(Clone, Copy, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new<T: Shape + 'a>(t: f64, object: &'a T) -> Self {
        Self { t, object }
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object == other.object
    }
}
pub trait IntersectVec {
    fn hit(&self) -> Option<Intersection<'_>>;
}

impl IntersectVec for Vec<Intersection<'_>> {
    fn hit(&self) -> Option<Intersection<'_>> {
        // Performance wise, this is probably not great, but meh.
        self.iter()
            .filter(|&&x| x.t >= 0.0)
            .min_by(|&&a, &&b| a.t.total_cmp(&b.t))
            .copied()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        math::{
            matrix::Matrix,
            tuple::{Tuple},
        },
        ray::IntersectVec,
        shape::sphere::Sphere,
    };

    use super::{Intersection, Ray};

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
    fn hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = vec![i2, i1];

        assert_eq!(xs.hit().expect("should exist"), i1);
    }

    #[test]
    fn hit_between() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);

        let xs = vec![i2, i1];

        assert_eq!(xs.hit().expect("should exist"), i2);
    }
    #[test]
    fn hit_behind() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);

        let xs = vec![i2, i1];

        assert_eq!(xs.hit(), None)
    }

    #[test]
    fn hit_2() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-1.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let xs = vec![i1, i2, i3, i4];

        assert_eq!(xs.hit().expect("should exist"), i4)
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
