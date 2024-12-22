use raytracer::math::tuple::Tuple;

fn main() {
    let environ = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(0.2, 0.0, 0.0),
    };

    let mut proj = Projectile {
        position: Tuple::pointi(0, 1, 0),
        velocity: Tuple::vectori(1, 1, 0).normalize(),
    };

    loop {
        proj = tick(&environ, &proj);
        println!("{:?}", proj.position);
        if proj.position.y <= 0.0 {
            break;
        }
    }
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
