use std::fs;

use raytracer::{
    canvas::Canvas,
    colour::Colour,
    lights::PointLight,
    math::{
        matrix::Matrix,
        tuple::{pointi, Tuple},
    },
    ray::{Ray, RayIntersect},
    shape::sphere::Sphere,
};

fn main() -> std::result::Result<(), std::io::Error> {
    let mut canvas = Canvas::new_with_colour(400, 400, Colour::BLACK);
    let ray_origin = Tuple::pointi(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas.width as f64;
    let wall_half = wall_size / 2.0;

    let mut sphere =
        Sphere::new_with_transform(Matrix::scaling(1.0, 0.5, 1.0).rotate_z(45_f64.to_radians()));

    sphere.material.colour = Colour::new(1.0, 0.2, 1.0);

    let light = PointLight::new(Colour::newi(1, 1, 1), pointi(-10, 10, -10));

    for row_p in 0..canvas.height {
        let world_y = wall_half - pixel_size * row_p as f64;
        for col_p in 0..canvas.width {
            let world_x = -wall_half + pixel_size * col_p as f64;

            let pos = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (pos - ray_origin).normalize());

            if let Some(xs) = sphere.intersect(ray) {
                let first = xs.first().unwrap();
                let point = ray.position(first.t);
                let normal = first.object.normal_at(point);
                let eye = -ray.direction;
                canvas[(row_p, col_p)] =
                    first.object.material().lighting(light, point, eye, normal);
            }
        }
    }

    fs::write("out/shadow_lit_squished.ppm", canvas.into_ppm())
}
