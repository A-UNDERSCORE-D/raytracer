use crate::{
    colour::Colour,
    intersection::{IntersectVec, Intersection, IntersectionComputions},
    lights::{Light, PointLight},
    materials::Material,
    math::{
        matrix::Matrix,
        tuple::{pointi, Tuple},
    },
    ray::{Ray, RayIntersect},
    shape::{sphere::Sphere, Shape},
};

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub light: Vec<Box<dyn Light>>,
}

// SAFETY: Safe because we only ever read from Shape and Light after construct.
// This does however imply that Shape and Light are always safe to read from using their methods.
// we can be "sure" of this because Shape and Light dont have Mut references.
unsafe impl Send for World {}
unsafe impl Sync for World {}

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

    pub fn shade_hit(&self, comps: IntersectionComputions) -> Colour {
        let count = self.light.len() as f64;
        self.light
            .iter()
            .map(|l| {
                comps.object.material().lighting(
                    &**l,
                    comps.over_point,
                    comps.eye_vector,
                    comps.normal_vector,
                    self.is_shadowed_by(&**l, comps.over_point),
                )
            })
            .reduce(|acc, c| acc + (c / count))
            .unwrap()
    }

    pub fn colour_at(&self, ray: Ray) -> Colour {
        let xs = self.intersect_world(ray);
        let hit = xs.hit();

        if xs.hit().is_none() {
            return Colour::BLACK;
        }

        let hit = hit.unwrap();

        self.shade_hit(hit.prepare_computations(ray))
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        self.light.iter().any(|l| self.is_shadowed_by(&**l, point))
    }

    fn is_shadowed_by(&self, light: &dyn Light, point: Tuple) -> bool {
        let v = *light.position() - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let xs = self.intersect_world(Ray::new(point, direction));

        let hit = xs.hit();

        hit.is_some_and(|hits| hits.t < distance)
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
            light: vec![Box::new(PointLight::new(
                Colour::newi(1, 1, 1),
                pointi(-10, 10, -10),
            ))],
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

        assert_eq!(w.light[0].intensity(), &Colour::newi(1, 1, 1));
        assert_eq!(w.light[0].position(), &pointi(-10, 10, -10));
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
    mod shading {
        use crate::{
            intersection::Intersection, lights::PointLight, math::tuple::point,
            shape::sphere::Sphere,
        };

        use super::*;

        #[test]
        fn outside() {
            let w = World::default();
            let ray = Ray::new(pointi(0, 0, -5), vectori(0, 0, 1));
            let shape = &*w.objects[0];
            let i = Intersection::new(4.0, shape);

            let comps = i.prepare_computations(ray);
            let c = w.shade_hit(comps);

            assert_eq!(c, Colour::new(0.38066, 0.47583, 0.2855));
        }

        #[test]
        fn inside() {
            let w = World {
                light: vec![Box::new(PointLight::new(
                    Colour::WHITE,
                    point(0.0, 0.25, 0.0),
                ))],
                ..World::default()
            };
            let ray = Ray::new(pointi(0, 0, 0), vectori(0, 0, 1));
            let shape = &*w.objects[1];
            let i = Intersection::new(0.5, shape);

            let comps = i.prepare_computations(ray);
            let c = w.shade_hit(comps);

            assert_eq!(c, Colour::new(0.90498, 0.90498, 0.90498))
        }

        #[test]
        fn shadowed_hit() {
            let w = World {
                light: vec![PointLight::new_boxed(Colour::WHITE, pointi(0, 0, -10))],
                objects: vec![
                    Box::new(Sphere::default()),
                    Box::new(Sphere::new_with_transform(Matrix::translationi(0, 0, 10))),
                ],
            };

            let r = Ray::new(pointi(0, 0, 5), vectori(0, 0, 1));
            let i = Intersection::new(4.0, &*w.objects[1]);

            let comps = i.prepare_computations(r);

            assert_eq!(w.shade_hit(comps), Colour::new(0.1, 0.1, 0.1));
        }

        mod colour_at {
            use crate::{materials::Material, shape::sphere::Sphere};

            use super::*;

            #[test]
            fn miss() {
                let w = World::default();
                let r = Ray::new(pointi(0, 0, -5), vectori(0, 1, 0));

                assert_eq!(w.colour_at(r), Colour::BLACK)
            }

            #[test]
            fn hit() {
                let w = World::default();
                let r = Ray::new(pointi(0, 0, -5), vectori(0, 0, 1));

                assert_eq!(w.colour_at(r), Colour::new(0.38066, 0.47583, 0.2855))
            }

            #[test]
            fn hit_behind() {
                let w = World {
                    objects: vec![
                        Box::new(Sphere::new_with_material(Material {
                            ambient: 1.0,
                            ..Material::default()
                        })),
                        Box::new(Sphere::new_with_material(Material {
                            ambient: 1.0,
                            ..Material::default()
                        })),
                    ],
                    ..Default::default()
                };
                let ray = Ray::new(point(0.0, 0.0, 0.75), vectori(0, 0, -1));

                assert_eq!(w.colour_at(ray), w.objects[1].material().colour)
            }
        }

        mod shadow {
            use super::*;

            macro_rules! shadow_test {
                ($name:ident, $point:expr, $expected:expr) => {
                    #[test]
                    fn $name() {
                        let w = World::default();
                        assert_eq!(w.is_shadowed($point), $expected)
                    }
                };
            }

            shadow_test!(unshadowed, pointi(0, 10, 0), false);
            shadow_test!(shadowed, pointi(10, -10, 10), true);
            shadow_test!(behind_light, pointi(-20, 20, -20), false);
            shadow_test!(between_light_object, pointi(-2, 2, -2), false);
        }
    }
}
