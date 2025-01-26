use std::fs;

use raytracer::{
    canvas::Canvas,
    colour::Colour,
    math::{matrix::Matrix, tuple::Tuple},
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

    let sphere = Sphere::new_with_transform(
        Matrix::shearing(1.0, 0.0, 0.5, 0.0, 0.5, 0.0)
            .rotate_x(45_f64.to_radians())
            .scale(0.1, 1.0, 1.0),
    );

    for row_p in 0..canvas.height {
        let world_y = wall_half - pixel_size * row_p as f64;
        for col_p in 0..canvas.width {
            let world_x = -wall_half + pixel_size * col_p as f64;

            let pos = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (pos - ray_origin).normalize());

            if sphere.intersect(ray).is_some() {
                canvas[(row_p, col_p)] = Colour::RED;
            }
        }
    }

    fs::write("out/shadow.ppm", canvas.into_ppm())
}
