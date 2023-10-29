use approx::abs_diff_eq;

use super::Tuple;

#[derive(Debug)]
pub struct Matrix {
    width: usize,
    height: usize,
    data: Vec<f64>,
}

// ------------------------------------------------------
impl Matrix {
    pub fn new(width: usize, height: usize, default_value: f64) -> Self {
        Matrix {
            width,
            height,
            data: vec![default_value; width * height],
        }
    }

    pub fn new2() -> Self {
        Self::new(2, 2, 0.0)
    }

    pub fn new3() -> Self {
        Self::new(3, 3, 0.0)
    }

    pub fn new4() -> Self {
        Self::new(4, 4, 0.0)
    }

    pub fn identity() -> Matrix{
        let mut m = Self::new4();
        m[0][0] = 1.0;
        m[1][1] = 1.0;
        m[2][2] = 1.0;
        m[3][3] = 1.0;
        m
    }

    pub fn init(self, vals: Vec<f64>) -> Self {
        Matrix { data: vals, ..self }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.width, self.height, 0.0);
        for row in 0..self.width {
            for col in 0..self.height {
                result[row][col] = self[col][row];
            }
        }
        result
    }

    pub fn determinant(&self) -> f64 {
        if self.width == 2 {
            return self[0][0] * self[1][1] - self[0][1] * self[1][0];
        }
        
        let mut det: f64 = 0.0;
        for col in 0..self.width {
            det += self[0][col] * self.cofactor(0, col);
        }
        det
    }

    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix {
        let mut result = Matrix::new(self.width - 1, self.height - 1, 0.0);
        let mut rd: usize = 0;
        for r in 0..self.width {
            if r == row {
                rd = 1;
                continue;
            }

            let mut cd: usize = 0;
            for c in 0..self.height {
                if c == col {
                    cd = 1;
                    continue;
                }

                // println!("row = {}, col = {}, r = {}, c = {}, rd = {}, cd = {}", row, col, r, c, rd, cd);
                result[r - rd][c - cd] = self[r][c];
            }
        }
        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.sub_matrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let m = self.minor(row, col);
        if (row + col) % 2 == 0 {
            m
        } else {
            -m
        }
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix> {
        if !self.is_invertible() {
            return None;
        }

        let mut m2 = Matrix::new(self.width, self.height, 0.0);
        let det = self.determinant();

        for row in 0..self.width {
            for col in 0..self.height {
                let c = self.cofactor(row, col);

                // Note that "col, row" here, instead of "row, col",
                // accomplishes a transpose operation.
                m2[col][row] = c / det;
            }
        }
        Some(m2)
    }
}

// ------------------------------------------------------
impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.width == other.width && self.height == other.height {
            for idx in 0..self.data.len() {
                // Lower precision compare, setting epsilon.
                if !abs_diff_eq!(self.data[idx], other.data[idx], epsilon = 1.0e-3) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
}

// ------------------------------------------------------
impl std::ops::Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, row: usize) -> &[f64] {
        let start = row * self.width;
        &self.data[start..start + self.width]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut [f64] {
        let start = row * self.width;
        &mut self.data[start..start + self.width]
    }
}

// ------------------------------------------------------
impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut result = Matrix::new(self.width, self.height, 0.0);

        for row in 0..self.width {
            for col in 0..self.height {
                result[row][col] = self[row][0] * rhs[0][col] +
                    self[row][1] * rhs[1][col] +
                    self[row][2] * rhs[2][col] +
                    self[row][3] * rhs[3][col];
            }
        }

        result
    }
}

impl<T> std::ops::Mul<T> for Matrix
where
    T: Tuple,
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(
            self[0][0] * rhs.x()
                + self[0][1] * rhs.y()
                + self[0][2] * rhs.z()
                + self[0][3] * rhs.w(),
            self[1][0] * rhs.x()
                + self[1][1] * rhs.y()
                + self[1][2] * rhs.z()
                + self[1][3] * rhs.w(),
            self[2][0] * rhs.x()
                + self[2][1] * rhs.y()
                + self[2][2] * rhs.z()
                + self[2][3] * rhs.w(),
        )
    }
}

// ------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::primitives::{Point, Tuple};

    use super::*;

    #[test]
    fn matrix_construction() {
        let data = [
            1.0, 2.0, 3.0, 4.0, 
            5.5, 6.5, 7.5, 8.5, 
            9.0, 10.0, 11.0, 12.0, 
            13.5, 14.5, 15.5, 16.5,
        ];

        let m = Matrix::new4().init(data.to_vec());

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn matrix2_construction() {
        let m = Matrix::new2().init([
            -3.0, 5.0,
            1.0, -2.0
        ].to_vec());

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn matrix3_construction() {
        let m = Matrix::new3().init([
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0
        ].to_vec());

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix_eq() {
        let a = Matrix::new4().init([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ].to_vec());

        let b = Matrix::new4().init([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ].to_vec());

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_ne() {
        let a = Matrix::new4().init([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ].to_vec());

        let b = Matrix::new4().init([
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0
        ].to_vec());

        assert_ne!(a, b);
    }

    #[test]
    fn matrix_mult_two_matrices() {
        let a = Matrix::new4().init([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ].to_vec());

        let b = Matrix::new4().init([
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0
        ].to_vec());

        let c = a * b;
        
        let expected = Matrix::new4().init([
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0
        ].to_vec());

        assert_eq!(expected, c);
    }

    #[test]
    fn matrix_mult_tuple() {
        let a = Matrix::new4().init([
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0
        ].to_vec());

        let p = Point::new(1.0, 2.0, 3.0);
        let result = a * p;
        let expected = Point::new(18.0, 24.0, 33.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_mult_by_identity() {
        let a = Matrix::new4().init([
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0
        ].to_vec());

        let result = a * Matrix::identity();

        //TODO(feg): can't compare a to result -  borrow checker...

        let expected = Matrix::new4().init([
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0
        ].to_vec());

        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_mult_identity_by_tuple() {
        let p = Point::new(1.0, 2.0, 3.0);
        let result = Matrix::identity() * p;
        //TODO(feg): tdod p * matrix
        assert_eq!(p, result);
    }

    #[test]
    fn matrix_transpose() {
        let a = Matrix::new4().init([
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0
        ].to_vec());

        let result= a.transpose();

        let expected = Matrix::new4().init([
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0
        ].to_vec());

        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_determinant_2x2() {
        let a = Matrix::new2().init([
            1.0, 5.0,
            -3.0, 2.0
        ].to_vec());
        
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn matrix_submatrix() {
        let a = Matrix::new3().init([
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0
        ].to_vec());

        let s = a.sub_matrix(0, 2);

        let expected = Matrix::new2().init([
            -3.0, 2.0,
            0.0, 6.0
        ].to_vec());

        assert_eq!(expected, s);

        let a = Matrix::new4().init([
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        ].to_vec());

        let s = a.sub_matrix(2, 1);

        let expected = Matrix::new3().init([
            -6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0
        ].to_vec());

        assert_eq!(expected, s);
    }

    #[test]
    fn matrix_minor() {
        let a = Matrix::new3().init([
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0
        ].to_vec());

        let b = a.sub_matrix(1, 0);

        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn matrix_cofactor() {
        let a = Matrix::new3().init([
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0
        ].to_vec());

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn matrix_determinant() {
        let m3 = Matrix::new3().init([
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0
        ].to_vec());

        assert_eq!(m3.cofactor(0, 0), 56.0);
        assert_eq!(m3.cofactor(0, 1), 12.0);
        assert_eq!(m3.cofactor(0, 2), -46.0);
        assert_eq!(m3.determinant(), -196.0);

        let m4 = Matrix::new4().init([
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        ].to_vec());

        assert_eq!(m4.cofactor(0, 0), 690.0);
        assert_eq!(m4.cofactor(0, 1), 447.0);
        assert_eq!(m4.cofactor(0, 2), 210.0);
        assert_eq!(m4.cofactor(0, 3), 51.0);
        assert_eq!(m4.determinant(), -4071.0);
    }

    #[test]
    fn matrix_is_invertible() {
        let m4 = Matrix::new4().init([
            6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0
        ].to_vec());

        assert_eq!(m4.determinant(), -2120.0);
        assert_eq!(m4.is_invertible(), true);

        let m4 = Matrix::new4().init([
            -4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0
        ].to_vec());

        assert_eq!(m4.determinant(), 0.0);
        assert_eq!(m4.is_invertible(), false);
    }

    #[test]
    fn matrex_inverse() {
        let m4 = Matrix::new4().init([
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0
        ].to_vec());

        match m4.inverse() {
            Some(result) => {
                assert_eq!(m4.determinant(), 532.0);
                assert_eq!(m4.cofactor(2, 3), -160.0);
                assert_eq!(result[3][2], -160.0 / 532.0);
                assert_eq!(m4.cofactor(3, 2), 105.0);
                assert_eq!(result[2][3], 105.0 / 532.0);

                let expected = Matrix::new4().init([
                    0.21805, 0.45113, 0.24060, -0.04511,
                    -0.80827, -1.45677, -0.44361, 0.52068,
                    -0.07895, -0.22368, -0.05263, 0.19737,
                    -0.52256, -0.81391, -0.30075, 0.30639
                ].to_vec());

                assert_eq!(expected, result);
            },
            None => assert!(false)
        }
    }

    #[test]
    fn matrex_inverse2() {
        let m4 = Matrix::new4().init([
            8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0
        ].to_vec());

        match m4.inverse() {
            Some(result) => {
                let expected = Matrix::new4().init([
                    -0.15385, -0.15385, -0.28205, -0.53846,
                    -0.07692, 0.12308, 0.02564, 0.03077,
                    0.35897, 0.35897, 0.43590, 0.92308,
                    -0.69231, -0.69231, -0.76923, -1.92308
                ].to_vec());

                assert_eq!(expected, result);
            },
            None => assert!(false)
        }
    }

    #[test]
    fn matrex_inverse3() {
        let m4 = Matrix::new4().init([
            9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0
        ].to_vec());

        match m4.inverse() {
            Some(result) => {
                let expected = Matrix::new4().init([
                    -0.04074, -0.07778, 0.14444, -0.22222,
                    -0.07778, 0.03333, 0.36667, -0.33333,
                    -0.02901, -0.14630, -0.10926, 0.12963,
                    0.17778, 0.06667, -0.26667, 0.33333
                ].to_vec());

                assert_eq!(expected, result);
            },
            None => assert!(false)
        }
    }

    #[test]
    fn matrix_mult_product_by_inverse() {
        let a = Matrix::new4().init([
            3.0, -9.0, 7.0, 3.0,
            3.0, -8.0, 2.0, -9.0,
            -4.0, 4.0, 4.0, 1.0,
            -6.0, 5.0, -1.0, 1.0
        ].to_vec());

        let b = Matrix::new4().init([
            8.0, 2.0, 2.0, 2.0,
            3.0, -1.0, 7.0, 0.0,
            7.0, 0.0, 5.0, 4.0,
            6.0, -2.0, 0.0, 5.0
        ].to_vec());

        match b.inverse() {
            Some(binv) => {
                let c = a * b;

                //TODO(feg): Unable to use a from above due to borrow check.
                let a = Matrix::new4().init([
                    3.0, -9.0, 7.0, 3.0,
                    3.0, -8.0, 2.0, -9.0,
                    -4.0, 4.0, 4.0, 1.0,
                    -6.0, 5.0, -1.0, 1.0
                ].to_vec());

                assert_eq!(c * binv, a);
            },
            None => assert!(false)
        }
    }

}
