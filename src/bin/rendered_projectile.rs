use std::fs::{self};

use raytracer::{canvas::Canvas, colour::Colour, math::tuple::Tuple};

fn main() {
    let environ = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut proj = Projectile {
        position: Tuple::pointi(0, 1, 0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let mut canvas = Canvas::new(900, 550);

    loop {
        proj = tick(&environ, &proj);
        println!("{:?}", proj.position);
        if (0.0..canvas.width as f64).contains(&proj.position.x)
            && (0.0..canvas.height as f64).contains(&proj.position.y)
        {
            let height = canvas.height;
            canvas[(
                proj.position.x.floor() as usize,
                height - proj.position.y.floor() as usize,
            )] = Colour::RED;
        }

        if proj.position.y <= 0.0 {
            break;
        }
    }

    fs::write("out/projectile_rendered.ppm", canvas.into_ppm()).unwrap();
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let pos = projectile.position + projectile.velocity;
    let vel = projectile.velocity + environment.gravity + environment.wind;

    Projectile {
        position: pos,
        velocity: vel,
    }
}
