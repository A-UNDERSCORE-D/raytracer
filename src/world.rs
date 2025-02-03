use crate::{
    colour::Colour,
    intersection::Intersection,
    lights::{Light, PointLight},
    materials::Material,
    math::{matrix::Matrix, tuple::pointi},
    ray::Ray,
    shape::{sphere::Sphere, Shape},
};

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub light: Box<dyn Light>,
}

impl World {
    pub fn intersect_world(&self, ray: Ray) -> Vec<Intersection> {
        let mut xs: Vec<_> = self
            .objects
            .iter()
            .flat_map(|s| s.intersect(ray).unwrap_or_default())
            .collect();

        xs.sort_by(|a, b| a.t.total_cmp(&b.t));
        xs
    }
}

impl Default for World {
    fn default() -> Self {
        let s1_mat = Material {
            colour: Colour::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        World {
            objects: vec![
                Box::new(Sphere::new_with_material(s1_mat)),
                Box::new(Sphere::new_with_transform(Matrix::scaling(0.5, 0.5, 0.5))),
            ],
            light: Box::new(PointLight::new(Colour::newi(1, 1, 1), pointi(-10, 10, -10))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        colour::Colour,
        math::{
            matrix::Matrix,
            tuple::{pointi, vectori},
        },
        ray::Ray,
        world::World,
    };

    #[test]
    fn default() {
        let w: World = Default::default();

        assert_eq!(w.objects[0].material().diffuse, 0.7);
        assert_eq!(w.objects[0].material().specular, 0.2);
        assert_eq!(w.objects[0].material().colour, Colour::new(0.8, 1.0, 0.6));

        assert_eq!(w.objects[1].transform(), &Matrix::scaling(0.5, 0.5, 0.5));

        assert_eq!(w.light.intensity(), &Colour::newi(1, 1, 1));
        assert_eq!(w.light.position(), &pointi(-10, 10, -10));
    }

    #[test]
    fn intersect_world() {
        let world = World::default();
        let ray = Ray::new(pointi(0, 0, -5), vectori(0, 0, 1));

        let xs = world.intersect_world(ray);

        assert_eq!(xs.len(), 4);

        let expected = vec![4.0, 4.5, 5.5, 6.0];

        for (i, (got, want)) in xs.iter().zip(expected).enumerate() {
            assert_eq!(got.t, want, "broke for {i}")
        }
    }
}
