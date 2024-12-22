use std::ops::{Index, IndexMut};

use crate::colour::Colour;

/// A canvas using a Vec as a backing store.
#[derive(Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<Colour>,
}

impl Canvas {
    fn make_index(width: usize, x: usize, y: usize) -> usize {
        (width * y) + x
    }

    /// Create a new canvas with the given extents
    /// ```
    /// # use raytracer::canvas::Canvas;
    /// let canvas = Canvas::new(10, 10);
    /// assert_eq!(canvas.iter().count(), 100);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Colour::default(); width * height],
        }
    }

    pub fn new_with_colour(width: usize, height: usize, base_colour: Colour) -> Self {
        Self {
            width,
            height,
            data: vec![base_colour; width * height],
        }
    }

    /// Access the underlying vector directly (Note this is NOT a mutable version)
    pub fn vec(&self) -> &Vec<Colour> {
        &self.data
    }

    /// Access the data within this canvas as an iterator. Due to the layout, this
    /// will step through data from 0,0 to max,max row by row
    pub fn iter(&self) -> impl Iterator<Item = &Colour> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Colour> {
        self.data.iter_mut()
    }
}

/// PPM tasks
impl Canvas {
    pub fn into_ppm(&self) -> String {
        let mut out = format!("P3\n{} {}\n255\n", self.width, self.height).to_owned();
        let stream = self
            .iter()
            .map(Colour::to_ppm)
            .flat_map(|s| s.split_whitespace().map(&str::to_owned).collect::<Vec<_>>());

        let mut size = 0;
        for (i, c) in stream.enumerate() {
            let next_len = size + c.len() + 1;
            if next_len >= 70 || i % (self.width * 3) == 0 {
                size = 0;
                out.pop();
                out.push('\n');
            }

            size += c.len() + 1;
            out.push_str(c.as_str());
            out.push(' ');
        }
        out.pop(); // Drop trailing space
        out
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Colour;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[Canvas::make_index(self.width, index.0, index.1)]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[Canvas::make_index(self.width, index.0, index.1)]
    }
}

#[cfg(test)]
mod test {
    use crate::colour::Colour;

    use super::Canvas;

    #[test]
    fn index() {
        let c = Canvas::new(5, 5);
        assert!(c.vec().iter().all(|v| *v == Colour::default()));
    }

    #[test]
    fn iter() {
        let c = Canvas::new(10, 10);
        assert_eq!(c.iter().count(), 100);
        for n in c.iter() {
            assert!(*n == Colour::default())
        }
    }

    #[test]
    #[should_panic]
    fn index_oob() {
        Canvas::new(1, 1)[(1, 1)];
    }
    #[test]
    fn index_assign() {
        let mut c = Canvas::new(10, 10);

        c[(4, 4)] = Colour::newi(1, 2, 3);

        assert_eq!(c[(4, 4)], Colour::newi(1, 2, 3))
    }

    mod ppm {
        use crate::{canvas::Canvas, colour::Colour};

        #[test]
        fn header() {
            let ppm = Canvas::new(5, 3).into_ppm();
            let header: Vec<_> = ppm.lines().take(3).collect();
            assert_eq!(header[0], "P3");
            assert_eq!(header[1], "5 3");
            assert_eq!(header[2], "255")
        }

        #[test]
        fn data() {
            let mut c = Canvas::new(5, 3);
            c[(0, 0)] = Colour::new(1.5, 0.0, 0.0);
            c[(2, 1)] = Colour::new(0.0, 0.5, 0.0);
            c[(4, 2)] = Colour::new(-0.5, 0.0, 1.0);

            let ppm = c.into_ppm();
            let data: Vec<_> = ppm.lines().skip(3).take(3).collect();

            assert_eq!(data[0], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
            assert_eq!(data[1], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
            assert_eq!(data[2], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        }

        #[test]
        fn complex_data() {
            let c = Canvas::new_with_colour(10, 2, Colour::new(1.0, 0.8, 0.6));

            let ppm = c.into_ppm();
            let data: Vec<_> = ppm.lines().skip(3).collect();
            let expected: Vec<&str> =
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204          
                153 255 204 153 255 204 153 255 204 153 255 204 153          
                255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204          
                153 255 204 153 255 204 153 255 204 153 255 204 153"
                    .lines()
                    .map(&str::trim)
                    .collect();

            assert_eq!(data, expected)
        }
    }
}
