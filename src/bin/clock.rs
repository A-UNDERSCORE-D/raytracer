use std::fs;

use raytracer::{
    canvas::Canvas,
    colour::Colour,
    math::{matrix::IDENTITY_4X4, tuple::Tuple},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Canvas::new(100, 100);
    let twelve = Tuple::pointi(0, (canvas.width / 3) as i32, 0);

    let dots = 12;
    let step = 360.0 / dots as f64;

    for i in 0..dots {
        let n = IDENTITY_4X4
            .clone()
            .rotate_z((i as f64 * step).to_radians()) // z because we rotate *around* this axis
            .translate((canvas.height / 2) as f64, (canvas.width / 2) as f64, 0.0)
            * twelve;

        canvas[(n.x.round() as usize, n.y.round() as usize)] = Colour::WHITE
    }

    println!("Starting output");
    fs::write("out/clock.ppm", canvas.into_ppm())?;
    Ok(())
}
