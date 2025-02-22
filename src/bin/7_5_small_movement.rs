use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4},
    fs,
};

use raytracer::{
    camera::Camera,
    colour::Colour,
    lights::PointLight,
    materials::Material,
    math::{
        matrix::Matrix,
        tuple::{point, vectori, Tuple},
    },
    shape::{sphere::Sphere, Shape},
    world::World,
};

fn generate_range(start: f64, stop: f64, step: f64) -> Vec<f64> {
    let mut i = start;
    let mut out = vec![];
    while i < stop {
        out.push(i);
        i += step;
    }
    out
}

fn main() {
    let mut frames = vec![];
    for (n, i) in generate_range(0.0, 20.0, 0.2).into_iter().enumerate() {
        println!("on frame {n}");
        let from = if i > 10.0 {
            point(0.0, 1.5 + (i - 10.0), -15.0)
        } else {
            point(0.0, 1.5, -5.0 + (-i))
        };

        frames.push(render_image(100 * 10, 50 * 10, from));
    }

    frames
        .iter()
        .enumerate()
        .for_each(|(n, f)| fs::write(format!("out/frames/f_{:05}.ppm", n), f).unwrap());
}

fn render_image(hsize: usize, vsize: usize, from: Tuple) -> Vec<u8> {
    let world = make_scene();
    let camera = Camera::new_with_transform(
        hsize,
        vsize,
        FRAC_PI_3,
        Matrix::view_transform(
            from, //point(0.0, 1.5, -5.0),
            point(0.0, 1.0, 0.0),
            vectori(0, 1, 0),
        ),
    );

    camera.render_parallel(world).into_ppm_binary()
}

fn make_scene() -> World {
    let floor = Sphere::new(
        Matrix::scaling(10.0, 0.01, 10.0),
        Material {
            colour: Colour::new(1.0, 0.9, 0.9),
            specular: 0.0,
            ..Default::default()
        },
    );

    let left_wall = Sphere::new(
        Matrix::scaling(10.0, 0.01, 10.0)
            .rotate_x(FRAC_PI_2)
            .rotate_y(-FRAC_PI_4)
            .translate(0.0, 0.0, 5.0),
        floor.material,
    );
    let right_wall = Sphere::new(
        Matrix::scaling(10.0, 0.01, 10.0)
            .rotate_x(FRAC_PI_2)
            .rotate_y(FRAC_PI_4)
            .translate(0.0, 0.0, 5.0),
        floor.material,
    );

    let middle = Sphere::new(
        Matrix::translation(-0.5, 1.0, 0.5),
        Material {
            colour: Colour::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );

    let right = Sphere::new(
        Matrix::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5),
        Material {
            colour: Colour::new(0.5, 1.0, 0.1),
            ..middle.material
        },
    );

    let left = Sphere::new(
        Matrix::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75),
        Material {
            colour: Colour::new(1.0, 0.8, 0.1),
            ..right.material
        },
    );

    let light = PointLight::new(Colour::WHITE, point(-10.0, 10.0, -10.0));

    World {
        objects: vec![floor, left_wall, right_wall, middle, right, left]
            .into_iter()
            .map(|o| Box::new(o) as Box<dyn Shape>)
            .collect(),
        light: vec![Box::new(light)],
    }
}
