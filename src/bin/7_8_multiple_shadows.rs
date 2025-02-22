use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4},
    fs,
};

use raytracer::{
    camera::Camera,
    colour::Colour,
    lights::{Light, PointLight},
    materials::Material,
    math::{
        matrix::Matrix,
        tuple::{point, vectori, ZERO_POINT},
    },
    shape::{sphere::Sphere, Shape},
    world::World,
};

fn main() {
    let mul = 50;
    let world = make_scene();
    let camera = Camera::new_with_transform(
        100 * mul,
        50 * mul,
        FRAC_PI_3,
        Matrix::view_transform(
            point(-10.0, 8.5, -15.0),
            point(0.0, 1.0, 0.0),
            vectori(0, 1, 0),
        ),
    );

    let res = camera.render_parallel(world);

    fs::write("out/7_world_sync_BINARY.ppm", res.into_ppm_binary()).unwrap();
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

    let mut lights: Vec<Box<dyn Light>> = vec![];
    let mut objects = vec![floor, left_wall, right_wall, middle, right, left];

    for i in 0..8 {
        objects.push(Sphere::new_with_transform(
            Matrix::scaling(0.10, 0.10, 0.10)
                .translate(1.0, 0.0, 0.0)
                .rotate_y(((360.0 / 8.0) * i as f64).to_radians())
                .translate(0.0, 4.0, 0.0),
        ));

        lights.push(PointLight::new_boxed(
            Colour::WHITE,
            Matrix::translation(0.0, 0.0, 30.0)
                .rotate_y(((360.0 / 8.0) * i as f64).to_radians())
                .translate(0.0, 50.0, 0.0)
                * ZERO_POINT,
        ));
    }

    World {
        objects: objects
            .into_iter()
            .map(|o| Box::new(o) as Box<dyn Shape>)
            .collect(),
        light: lights,
    }
}
