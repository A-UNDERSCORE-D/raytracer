use std::{
    cell::OnceCell,
    iter::once,
    ops::{Index, IndexMut, Mul},
    str::FromStr,
    sync::{LazyLock, OnceLock},
    thread::panicking,
};

use super::{float, tuple::Tuple};

#[derive(Clone, Debug)]
pub struct Matrix {
    data: Vec<f64>,
    width: usize,
    height: usize,
}

pub struct Ref<'a> {
    data: &'a [f64],
    stride: usize,
    count: usize,
}

impl Index<usize> for Ref<'_> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.stride]
    }
}

impl Ref<'_> {
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        (0..self.count).map(|i| self.index(i))
    }
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with_data(width, height, vec![0.0; width * height])
    }

    pub const fn new_with_data(width: usize, height: usize, data: Vec<f64>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    fn make_index(width: usize, col: usize, row: usize) -> usize {
        (width * row) + col
    }

    pub fn col(&self, col: usize) -> Ref {
        Ref {
            data: &self.data[col..],
            stride: self.width,
            count: self.height,
        }
    }

    pub fn row(&self, row: usize) -> Ref {
        let start = self.width * row;
        Ref {
            data: &self.data[start..start + self.width],
            stride: 1,
            count: self.width,
        }
    }
}

pub static IDENTITY_4X4: LazyLock<Matrix> = LazyLock::new(|| Matrix {
    width: 4,
    height: 4,
    data: vec![
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ],
});

impl FromStr for Matrix {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<f64>> = s
            .lines()
            .map(|l| {
                l.trim()
                    .replace("| ", "")
                    .split_whitespace()
                    .flat_map(&str::parse)
                    .collect()
            })
            .collect();

        assert!(rows.iter().all(|r| r.len() == rows[0].len()));
        let width = rows[0].len();
        let height = rows.len();
        let data: Vec<f64> = rows.into_iter().flatten().collect();

        Ok(Self::new_with_data(width, height, data))
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.height, rhs.height);
        assert_eq!(self.width, rhs.width);
        let mut data = vec![0.0; self.height * self.width];

        for col in 0..self.width {
            for row in 0..self.height {
                data[col + (self.width * row)] = self
                    .row(row)
                    .iter()
                    .zip(rhs.col(col).iter())
                    .map(|(l, r)| l * r)
                    .sum();
            }
        }
        Matrix::new_with_data(self.width, self.height, data)
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        assert_eq!(self.height, 4, "Cannot multiply a non 4* matrix by a tuple");
        Tuple {
            x: rhs.dot(&self.row(0).into()),
            y: rhs.dot(&self.row(1).into()),
            z: rhs.dot(&self.row(2).into()),
            w: rhs.dot(&self.row(3).into()),
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self
                .data
                .iter()
                .zip(other.data.iter())
                .all(|(a, b)| float::equal(*a, *b))
    }
}

// TODO: Probably useful, From for Colour and Tuple

macro_rules! indexer {
    ($typ:ty, $self:ident, $index:ident, $impl:expr) => {
        impl Index<$typ> for Matrix {
            type Output = f64;
            fn index(&$self, $index: $typ) -> &Self::Output {
                &$impl
            }
        }

        impl IndexMut<$typ> for Matrix {
            fn index_mut(&mut $self, $index: $typ) -> &mut Self::Output {
                &mut $impl
            }
        }
    };
}

indexer!(
    (usize, usize),
    self,
    index,
    self.data[Self::make_index(self.width, index.1, index.0)]
);

indexer!(usize, self, index, self.data[index]);

#[cfg(test)]
mod test {
    use crate::math::tuple::Tuple;

    use super::Matrix;

    #[test]
    fn parse() {
        let m: Matrix = "\
|  1   |  2   |  3   |  4   |
|  5.5 |  6.5 |  7.5 |  8.5 |
|  9   | 10   | 11   | 12   |
| 13.5 | 14.5 | 15.5 | 16.5 |\
"
        .parse()
        .unwrap();

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
        assert_eq!(m[(3, 3)], 16.5);
    }

    #[test]
    fn parse_2x2() {
        let m: Matrix = "\
| -3 |  5 |
|  1 | -2 |\
"
        .parse()
        .unwrap();
        assert_eq!(m[0], -3.0);
        assert_eq!(m[1], 5.0);
        assert_eq!(m[2], 1.0);
        assert_eq!(m[3], -2.0);

        assert_eq!(m[(1, 0)], 1.0)
    }

    #[test]
    fn parse_3x3() {
        let m: Matrix = "\
| -3 |  5 |  0 |
|  1 | -2 | -7 |
|  0 |  1 |  1 |\
    "
        .parse()
        .unwrap();

        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    #[test]
    fn equal() {
        let left: Matrix = "\
| 1 | 2 | 3 | 4 |
| 5 | 6 | 7 | 8 |
| 9 | 8 | 7 | 6 |
| 5 | 4 | 3 | 2 |\
"
        .parse()
        .unwrap();

        let right: Matrix = "\
| 1 | 2 | 3 | 4 |
| 5 | 6 | 7 | 8 |
| 9 | 8 | 7 | 6 |
| 5 | 4 | 3 | 2 |\
"
        .parse()
        .unwrap();

        assert_eq!(left, right)
    }

    #[test]
    fn neq() {
        let left: Matrix = "\
| 1 | 2 | 3 | 4 |
| 5 | 6 | 7 | 8 |
| 9 | 8 | 7 | 6 |
| 5 | 4 | 3 | 2 |\
"
        .parse()
        .unwrap();

        let right: Matrix = "\
| 2 | 3 | 4 | 5 |
| 6 | 7 | 8 | 9 |
| 8 | 7 | 6 | 5 |
| 4 | 3 | 2 | 1 |\
"
        .parse()
        .unwrap();

        assert_ne!(left, right)
    }

    #[test]
    fn row2_test() {
        let m = Matrix::new_with_data(2, 2, vec![0.0, 1.0, 2.0, 3.0]);

        assert_eq!(m.row(0).iter().copied().collect::<Vec<_>>(), vec![0.0, 1.0]);
        assert_eq!(m.row(1).iter().copied().collect::<Vec<_>>(), vec![2.0, 3.0]);
    }

    #[test]
    fn col2_test() {
        let m = Matrix::new_with_data(2, 2, vec![0.0, 1.0, 2.0, 3.0]);
        assert_eq!(m.col(0).iter().copied().collect::<Vec<_>>(), vec![0.0, 2.0]);
        assert_eq!(m.col(1).iter().copied().collect::<Vec<_>>(), vec![1.0, 3.0]);
    }

    #[test]
    fn mul() {
        let a: Matrix = "\
| 1 | 2 | 3 | 4 |
| 5 | 6 | 7 | 8 |
| 9 | 8 | 7 | 6 |
| 5 | 4 | 3 | 2 |"
            .parse()
            .unwrap();
        let b: Matrix = "\
| -2 | 1 | 2 |  3 |
|  3 | 2 | 1 | -1 |
|  4 | 3 | 6 |  5 |
|  1 | 2 | 7 |  8 |"
            .parse()
            .unwrap();
        let expected: Matrix = "\
| 20|  22 |  50 |  48 |
| 44|  54 | 114 | 108 |
| 40|  58 | 110 | 102 |
| 16|  26 |  46 |  42 |"
            .parse()
            .unwrap();

        assert_eq!(a * b, expected)
    }

    #[test]
    fn mul_tuple() {
        let a: Matrix = "\
| 1 | 2 | 3 | 4 |
| 2 | 4 | 4 | 2 |
| 8 | 6 | 4 | 1 |
| 0 | 0 | 0 | 1 |"
            .parse()
            .unwrap();

        let b = Tuple::pointi(1, 2, 3); // 1

        assert_eq!(a * b, Tuple::pointi(18, 24, 33))
    }
}
