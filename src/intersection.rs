use crate::{math::tuple::Tuple, ray::Ray, shape::Shape};

#[derive(Clone, Copy, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

pub struct IntersectionComputions<'a> {
    pub object: &'a dyn Shape,
    pub t: f64,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn prepare_computations(&self, ray: Ray) -> IntersectionComputions<'a> {
        let point = ray.position(self.t);
        let normal_vector = self.object.normal_at(point);
        let eye_vector = -ray.direction;
        let inside = normal_vector.dot(&eye_vector) < 0.0;

        IntersectionComputions {
            object: self.object,
            t: self.t,
            point,
            eye_vector,
            normal_vector: if inside {
                -normal_vector
            } else {
                normal_vector
            },
            inside,
        }
    }
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Shape) -> Self {
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
    use crate::shape::sphere::Sphere;

    use super::*;
    #[test]
    fn hit() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = vec![i2, i1];

        assert_eq!(xs.hit().expect("should exist"), i1);
    }

    #[test]
    fn hit_between() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);

        let xs = vec![i2, i1];

        assert_eq!(xs.hit().expect("should exist"), i2);
    }
    #[test]
    fn hit_behind() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);

        let xs = vec![i2, i1];

        assert_eq!(xs.hit(), None)
    }

    #[test]
    fn hit_2() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-1.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let xs = vec![i1, i2, i3, i4];

        assert_eq!(xs.hit().expect("should exist"), i4)
    }

    mod computations {
        use crate::math::tuple::{pointi, vectori};

        use super::*;

        #[test]
        fn precompute() {
            let r = Ray::new(pointi(0, 0, -5), vectori(0, 0, 1));
            let shape = Sphere::default();
            let intersection = Intersection::new(4.0, &shape);

            let comps = intersection.prepare_computations(r);

            assert_eq!(comps.object, &shape);
            assert_eq!(comps.t, intersection.t);
            assert_eq!(comps.point, pointi(0, 0, -1));
            assert_eq!(comps.eye_vector, vectori(0, 0, -1));
            assert_eq!(comps.normal_vector, vectori(0, 0, -1));
        }

        #[test]
        fn precompute_outside() {
            let ray = Ray::new(pointi(0, 0, -5), vectori(0, 0, 1));
            let s = &Sphere::default();
            let i = Intersection::new(4.0, s);

            let comps = i.prepare_computations(ray);

            assert!(!comps.inside);
        }

        #[test]
        fn precompute_inside() {
            let ray = Ray::new(pointi(0, 0, 0), vectori(0, 0, 1));
            let s = &Sphere::default();
            let i = Intersection::new(1.0, s);

            let comps = i.prepare_computations(ray);

            assert_eq!(comps.point, pointi(0, 0, 1));
            assert_eq!(comps.eye_vector, vectori(0, 0, -1));
            assert_eq!(comps.normal_vector, vectori(0, 0, -1)); // inverted you say?
            assert!(comps.inside);
        }
    }
}
