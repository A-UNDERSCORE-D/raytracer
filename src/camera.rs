use std::{
    sync::{mpsc, Arc},
    thread,
};

use crate::{
    canvas::Canvas,
    math::{
        matrix::{Matrix, IDENTITY_4X4},
        tuple::{point, ZERO_POINT},
    },
    ray::Ray,
    world::World,
};

#[derive(Clone)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64,
    pub transform: Matrix,
    // generated.
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
    pub inverse_transform: Matrix,
}

impl Camera {
    pub fn new_with_transform(hsize: usize, vsize: usize, fov: f64, transform: Matrix) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect_ratio = hsize as f64 / vsize as f64;
        let (half_width, half_height): (f64, f64);

        if aspect_ratio >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect_ratio;
        } else {
            half_width = half_view * aspect_ratio;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Self {
            hsize,
            vsize,
            fov,

            half_width,
            half_height,
            pixel_size,
            inverse_transform: transform.inverse().expect("Must be invertable."),

            transform, // Must go after the inverse because it moves :D
        }
    }

    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        Self::new_with_transform(hsize, vsize, fov, IDENTITY_4X4.clone())
    }
}

impl Camera {
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // offset from corner of canvas to center of pixel in world units
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // World-space coords, minus z (which is always camera+1)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = &self.inverse_transform * point(world_x, world_y, -1.0);
        let origin = &self.inverse_transform * ZERO_POINT;
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        // the image plane is 1 unit ahead of us, but lets start with width/height

        for x in 0..self.hsize {
            for y in 0..self.vsize {
                let ray = self.ray_for_pixel(x, y);
                canvas[(x, y)] = world.colour_at(ray);
            }
        }

        canvas
    }

    pub fn render_parallel(&self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        let (tx, rx) = mpsc::channel::<_>();

        let work: Vec<Vec<_>> = (0..self.hsize)
            .flat_map(|x| (0..self.vsize).map(move |y| (x, y)))
            .collect::<Vec<(usize, usize)>>()
            .chunks((self.hsize * self.vsize) / 16)
            .map(|x| x.to_owned())
            .collect();

        let world = Arc::new(world);

        for chunk in work {
            let tx = tx.clone();
            let s = self.clone();

            let world = world.clone();

            thread::spawn(move || {
                for (x, y) in chunk.iter().cloned() {
                    let ray = s.ray_for_pixel(x, y);
                    let c = world.colour_at(ray);
                    tx.send((x, y, c)).expect("Unable to send!");
                }
            });
        }

        drop(tx); // drop the "last" one; when all the threads exit we know we're done

        let mut count = 0;
        let total = self.hsize * self.vsize;
        while let Ok((x, y, c)) = rx.recv() {
            count += 1;
            if count % 1000 == 0 {
                print!("{count} / {total}\r");
            }
            canvas[(x, y)] = c;
        }

        canvas
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::FRAC_PI_2;

    use crate::{
        colour::Colour,
        math::{
            float,
            matrix::Matrix,
            tuple::{pointi, vectori},
        },
        world::World,
    };

    use super::Camera;

    #[test]
    fn pixel_size_horiz() {
        let c = Camera::new(200, 125, FRAC_PI_2);

        assert!(float::equal(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_vert() {
        let c = Camera::new(125, 200, FRAC_PI_2);

        assert!(float::equal(c.pixel_size, 0.01));
    }

    mod rays {
        use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};

        use crate::{
            camera::Camera,
            math::{
                matrix::Matrix,
                tuple::{point, vector},
            },
        };

        #[test]
        fn ray_center() {
            let c = Camera::new(201, 101, FRAC_PI_2);
            let r = c.ray_for_pixel(100, 50);

            assert_eq!(r.origin, point(0.0, 0.0, 0.0));
            assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
        }

        #[test]
        fn corner() {
            let c = Camera::new(201, 101, FRAC_PI_2);
            let r = c.ray_for_pixel(0, 0);

            assert_eq!(r.origin, point(0.0, 0.0, 0.0));
            assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
        }
        #[test]
        fn transformed() {
            let c = Camera::new_with_transform(
                201,
                101,
                FRAC_PI_2,
                Matrix::translation(0.0, -2.0, 5.0).rotate_y(FRAC_PI_4),
            );
            let r = c.ray_for_pixel(100, 50);

            assert_eq!(r.origin, point(0.0, 2.0, -5.0));
            assert_eq!(r.direction, vector(SQRT_2 / 2.0, 0.0, -(SQRT_2 / 2.0)));
        }
    }

    #[test]
    fn render() {
        let w: World = Default::default();
        let c = Camera::new_with_transform(
            11,
            11,
            FRAC_PI_2,
            Matrix::view_transform(pointi(0, 0, -5), pointi(0, 0, 0), vectori(0, 1, 0)),
        );

        let image = c.render(&w);

        assert_eq!(image[(5, 5)], Colour::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn render_parallel() {
        let w: World = Default::default();
        let c = Camera::new_with_transform(
            11,
            11,
            FRAC_PI_2,
            Matrix::view_transform(pointi(0, 0, -5), pointi(0, 0, 0), vectori(0, 1, 0)),
        );

        let image = c.render_parallel(w);

        assert_eq!(image[(5, 5)], Colour::new(0.38066, 0.47583, 0.2855))
    }
}
