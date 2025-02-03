use crate::shape::Shape;

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
    use crate::shape::sphere::Sphere;

    use super::*;
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
}
