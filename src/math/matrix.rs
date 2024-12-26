use std::{
    ops::{Index, IndexMut, Mul},
    str::FromStr,
    sync::LazyLock,
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

    pub fn new_with_data(width: usize, height: usize, data: Vec<f64>) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            width,
            height,
            data,
        }
    }

    pub fn new_with_datai(width: usize, height: usize, data: Vec<i32>) -> Self {
        Self::new_with_data(width, height, data.into_iter().map(f64::from).collect())
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

    // * Generators for specific types of matricies

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        todo!()
    }

    pub fn translationi(x: i32, y: i32, z: i32) -> Self {
        Self::translation(x as f64, y as f64, z as f64)
    }

    // * And here begins the more mathy functions...

    pub fn transpose(&self) -> Matrix {
        Self::new_with_data(
            self.width,
            self.height,
            (0..self.height)
                .flat_map(|i| self.col(i).iter().copied().collect::<Vec<_>>())
                .collect(),
        )
    }

    pub fn determinate(&self) -> f64 {
        match (self.width, self.height) {
            (2, 2) => (self[0] * self[3]) - (self[1] * self[2]),
            _ => self
                .row(0)
                .iter()
                .enumerate()
                .map(|(col, &v)| v * self.cofactor(0, col))
                .sum(),
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut data = Vec::with_capacity((self.width - 1) * (self.height - 1));
        for r in (0..self.height).filter(|&v| v != row) {
            for c in (0..self.width).filter(|&c| c != col) {
                data.push(self[(r, c)]);
            }
        }
        Matrix::new_with_data(self.width - 1, self.height - 1, data)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinate()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) & 1 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn can_invert(&self) -> bool {
        self.determinate() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix> {
        let determinate = self.determinate();

        if determinate == 0.0 {
            return None;
        }

        let mut out = Matrix::new(4, 4);
        for row in 0..self.height {
            for col in 0..self.width {
                let cofactor = self.cofactor(row, col);

                // Intentionally flipped
                out[(col, row)] = cofactor / determinate;
            }
        }

        Some(out)
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
                    .map(|(&l, &r)| l * r)
                    .sum();
            }
        }
        Matrix::new_with_data(self.width, self.height, data)
    }
}

impl Mul for &Matrix {
    type Output = Matrix;
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
    use crate::math::{matrix::IDENTITY_4X4, tuple::Tuple};

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

    #[test]
    fn identity() {
        let m: Matrix = "\
| 0 | 1 |  2 |  4 |
| 1 | 2 |  4 |  8 |
| 2 | 4 |  8 | 16 |
| 4 | 8 | 16 | 32 |"
            .parse()
            .unwrap();

        assert_eq!(&m * &*IDENTITY_4X4, m);
    }

    #[test]
    fn identity_tuple() {
        assert_eq!(
            IDENTITY_4X4.clone() // Clone required here because * moves the left hand side
                * Tuple {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                    w: 4.0,
                },
            Tuple {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0,
            }
        )
    }

    #[test]
    fn transpose_ident() {
        assert_eq!(IDENTITY_4X4.transpose(), *IDENTITY_4X4)
    }

    #[test]
    fn transpose() {
        let a: Matrix = "\
| 0 | 9 | 3 | 0 |
| 9 | 8 | 0 | 8 |
| 1 | 8 | 5 | 3 |
| 0 | 0 | 5 | 8 |"
            .parse()
            .unwrap();
        let expected: Matrix = "\
| 0 | 9 | 1 | 0 |
| 9 | 8 | 8 | 0 |
| 3 | 0 | 5 | 5 |
| 0 | 8 | 3 | 8 |"
            .parse()
            .unwrap();

        assert_eq!(a.transpose(), expected)
    }

    #[test]
    fn determinate() {
        let m = Matrix::new_with_data(2, 2, vec![1.0, 5.0, -3.0, 2.0]);

        assert_eq!(m.determinate(), 17.0)
    }

    #[test]
    fn submatrix() {
        let m = Matrix::new_with_datai(3, 3, vec![1, 5, 0, -3, 2, 7, 0, 6, -3]);
        let sub = Matrix::new_with_datai(2, 2, vec![-3, 2, 0, 6]);

        assert_eq!(m.submatrix(0, 2), sub)
    }

    #[test]
    fn submatrix_4x4() {
        let m = Matrix::new_with_datai(
            4,
            4,
            vec![-6, 1, 1, 6, -8, 5, 8, 6, -1, 0, 8, 2, -7, 1, -1, 1],
        );

        let expected = Matrix::new_with_datai(3, 3, vec![-6, 1, 6, -8, 8, 6, -7, -1, 1]);

        assert_eq!(m.submatrix(2, 1), expected)
    }

    #[test]
    fn minor() {
        let m = Matrix::new_with_datai(3, 3, vec![3, 5, 0, 2, -1, -7, 6, -1, 5]);
        let s = m.submatrix(1, 0);

        assert_eq!(m.minor(1, 0), s.determinate())
    }

    #[test]
    fn cofactor() {
        let m = Matrix::new_with_datai(3, 3, vec![3, 5, 0, 2, -1, -7, 6, -1, 5]);

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);

        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinate_3x3() {
        let m = Matrix::new_with_datai(3, 3, vec![1, 2, 6, -5, 8, -4, 2, 6, 4]);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinate(), -196.0);
    }

    #[test]
    fn determinate_4x4() {
        let m = Matrix::new_with_datai(
            4,
            4,
            vec![-2, -8, 3, 5, -3, 1, 7, 3, 1, 2, -9, 6, -6, 7, 7, -9],
        );

        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinate(), -4071.0)
    }

    #[test]
    fn inverse() {
        let a = Matrix::new_with_datai(
            4,
            4,
            vec![-5, 2, 6, -8, 1, -5, 1, 8, 7, 7, -6, -7, 1, -3, 7, 4],
        );
        let b = a.inverse().expect("This matrix should be invertable");
        let expected = Matrix::new_with_data(
            4,
            4,
            vec![
                0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
            ],
        );

        assert_eq!(a.determinate(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[(3, 2)], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[(2, 3)], 105.0 / 532.0);

        assert_eq!(b, expected);
    }

    #[test]
    fn inverse_2() {
        let m = Matrix::new_with_datai(
            4,
            4,
            vec![8, -5, 9, 2, 7, 5, 6, 1, -6, 0, 9, 6, -3, 0, -9, -4],
        );

        let expected = Matrix::new_with_data(
            4,
            4,
            vec![
                -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077,
                0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
            ],
        );

        assert_eq!(m.inverse().expect("must be invertable"), expected)
    }

    #[test]
    fn inverse_3() {
        let m = Matrix::new_with_datai(
            4,
            4,
            vec![9, 3, 0, 9, -5, -2, -6, -3, -4, 9, 6, 4, -7, 6, 6, 2],
        );

        let expected = Matrix::new_with_data(
            4,
            4,
            vec![
                -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333,
                -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
            ],
        );

        assert_eq!(m.inverse().expect("Must be invertab;e"), expected)
    }

    #[test]
    fn e2e_inversion() {
        let a = Matrix::new_with_datai(
            4,
            4,
            vec![3, -9, 7, 3, 3, -8, 2, -9, -4, 4, 4, 1, -6, 5, -1, 1],
        );
        let b =
            Matrix::new_with_datai(4, 4, vec![8, 2, 2, 2, 3, -1, 7, 0, 7, 0, 5, 4, 6, -2, 0, 5]);

        let c = &a * &b;
        assert_eq!(c * b.inverse().unwrap(), a)
    }

    mod translation {
        use super::*;
        #[test]
        fn simple() {
            let t = Matrix::translationi(5, -3, 2);
            let p = Tuple::pointi(-3, 4, 5);

            assert_eq!(t * p, Tuple::pointi(2, 1, 7))
        }
        #[test]
        fn inverse() {
            let inv = Matrix::translationi(5, -3, 2).inverse().unwrap();
            let p = Tuple::pointi(-3, 4, 5);
            assert_eq!(inv * p, Tuple::pointi(-8, 7, 3))
        }
    }
}
