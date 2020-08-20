use rand::distributions::Distribution;
use std::{fmt, ops};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut data: Vec<Vec<f64>> = Vec::new();
        for r in 0..rows {
            data.push(Vec::new());
            for _ in 0..cols {
                data[r].push(0_f64);
            }
        }

        Self { rows, cols, data }
    }

    pub fn to_string(&self) -> String {
        let mut display_string = String::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                display_string += &format!("{:.2}", self.data[i][j]);

                if j != self.cols - 1 {
                    display_string += ", ";
                }
            }

            if i != self.rows - 1 {
                display_string += "\n";
            }
        }

        display_string
    }

    pub fn fill<T: Into<f64>>(&mut self, n: T) {
        self.data = vec![vec![n.into(); self.cols]; self.rows];
    }

    pub fn fill_random(&mut self) {
        let mut rng = rand::thread_rng();
        let distribution = rand::distributions::Uniform::from(-0.3..0.3);
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.data[r][c] = distribution.sample(&mut rng);
            }
        }
    }

    pub fn transpose_mut(&mut self) {
        let transposed_matrix = self.transpose();
        self.rows = transposed_matrix.rows;
        self.cols = transposed_matrix.rows;
        self.data = transposed_matrix.data;
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);

        for r in 0..self.rows {
            for c in 0..self.cols {
                result.data[c][r] = self.data[r][c];
            }
        }

        result
    }

    pub fn add_col<T: Into<f64> + Copy>(&mut self, fill: T) {
        let n = fill.into();
        for r in 0..self.rows {
            self.data[r].push(n);
        }

        self.cols += 1;
    }

    pub fn add_row<T: Into<f64> + Copy>(&mut self, fill: T) {
        let n = fill.into();
        self.data.push(vec![n; self.cols]);
        self.rows += 1;
    }

    pub fn remove_col(&mut self) {
        for r in 0..self.rows {
            self.data[r].pop();
        }

        self.cols -= 1;
    }

    pub fn remove_row(&mut self) {
        self.data.pop();
        self.rows -= 1;
    }

    pub fn map(&self, f: &dyn Fn(f64) -> f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                result.data[r][c] = f(self.data[r][c]);
            }
        }

        result
    }

    pub fn map_mut(&mut self, f: &dyn Fn(f64) -> f64) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.data[r][c] = f(self.data[r][c]);
            }
        }
    }

    pub fn get_data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }

    pub fn rows(&self) -> usize {
        self.rows 
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn element_wise_multiply(&self, other: Matrix) -> Matrix {
        if self.rows == other.rows && self.cols == other.cols {
            let mut result = Matrix::new(self.rows, self.cols);

            for r in 0..self.rows {
                for c in 0..self.cols {
                    result.data[r][c] = self.data[r][c] * other.data[r][c];
                }
            }

            return result;
        } else {
            panic!("Matrices have different shapes! ({}, {}) and ({}, {})", self.rows, self.cols, other.rows, other.cols);
        }
    }

    pub fn element_wise_multiply_mut(&mut self, other: Matrix) {
        if self.rows == other.rows && self.cols == other.cols {
            for r in 0..self.rows {
                for c in 0..self.cols {
                    self.data[r][c] *= other.data[r][c];
                }
            }
        } else {
            panic!("Matrices have different shapes! ({}, {}) and ({}, {})", self.rows, self.cols, other.rows, other.cols);
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Implements matrix-to-matrix and matrix-to-scalar operations for all combinations of values and
// references
macro_rules! impl_matrix_ops {
    ($t1:ty, $t2:ty) => {
        impl ops::Add<$t2> for $t1 {
            type Output = Matrix;

            fn add(self, other: $t2) -> Matrix {
                if self.rows == other.rows && self.cols == other.cols {
                    let mut result = Matrix::new(self.rows, self.cols);

                    for r in 0..result.rows {
                        for c in 0..result.cols {
                            result.data[r][c] = self.data[r][c] + other.data[r][c]; 
                        }
                    }

                    return result;
                } else {
                    panic!("Matrices have different shapes! ({}, {}) and ({}, {})", self.rows, self.cols, other.rows, other.cols);
                }
            }
        }

        impl ops::Sub<$t2> for $t1 {
            type Output = Matrix;

            fn sub(self, other: $t2) -> Matrix {
                if self.rows == other.rows && self.cols == other.cols {
                    let mut result = Matrix::new(self.rows, self.cols);

                    for r in 0..result.rows {
                        for c in 0..result.cols {
                            result.data[r][c] = self.data[r][c] - other.data[r][c];
                        }
                    }

                    return result;
                } else {
                    panic!("Matrices have different shapes! ({}, {}) and ({}, {})", self.rows, self.cols, other.rows, other.cols);
                }
            }
        }

        impl ops::Mul<$t2> for $t1 {
            type Output = Matrix;

            fn mul(self, other: $t2) -> Matrix {
                if self.cols == other.rows {
                    let mut result = Matrix::new(self.rows, other.cols);

                    for r in 0..result.rows {
                        for c in 0..result.cols {
                            let mut sum = 0_f64;
                            for k in 0..self.cols {
                                sum += self.data[r][k] * other.data[k][c];
                            }
                            result.data[r][c] = sum;
                        }
                    }

                    return result;
                } else {
                    panic!(
                        "Matrices of size ({}, {}) and ({}, {}) cannot be multiplied",
                        self.rows, self.cols, other.rows, other.cols
                    );
                }
            }
        }
    };

    ($t: ty) => {
        impl ops::AddAssign<$t> for Matrix {
            fn add_assign(&mut self, other: $t) {
                if self.rows == other.rows && self.cols == other.cols {
                    for r in 0..self.rows {
                        for c in 0..self.cols {
                            self.data[r][c] += other.data[r][c];
                        }
                    }
                } else {
                    panic!("Matrices have different shapes! ({}, {}) and ({}, {})", self.rows, self.cols, other.rows, other.cols);
                }
            }
        }

        impl ops::SubAssign<$t> for Matrix {
            fn sub_assign(&mut self, other: $t) {
                if self.rows == other.rows && self.cols == other.cols {
                    for r in 0..self.rows {
                        for c in 0..self.cols {
                            self.data[r][c] -= other.data[r][c];
                        }
                    }
                } else {
                    panic!("Matrices have different shapes! ({}, {}) and ({}, {})", self.rows, self.cols, other.rows, other.cols);
                }
            }
        }

        impl ops::MulAssign<$t> for Matrix {
            fn mul_assign(&mut self, other: $t) {
                if self.cols == other.rows {
                    let mut result = Matrix::new(self.rows, other.cols);

                    for r in 0..result.rows {
                        for c in 0..result.cols {
                            let mut sum = 0_f64;
                            for k in 0..self.cols {
                                sum += self.data[r][k] * other.data[k][c];
                            }
                            result.data[r][c] = sum;
                        }
                    }
                    self.rows = result.rows;
                    self.cols = result.cols;
                    self.data = result.data;
                } else {
                    panic!(
                        "Matrices of size ({}, {}) and ({}, {}) cannot be multiplied",
                        self.rows, self.cols, other.rows, other.cols
                    );
                }
            }
        }

        impl<T: Into<f64>> ops::Add<T> for $t {
            type Output = Matrix;

            fn add(self, other: T) -> Matrix {
                let n = other.into();
                let mut result = Matrix::new(self.rows, self.cols);

                for r in 0..result.rows {
                    for c in 0..result.cols {
                        result.data[r][c] = self.data[r][c] + n;
                    }
                }

                result
            }
        }

        impl<T: Into<f64>> ops::Sub<T> for $t {
            type Output = Matrix;

            fn sub(self, other: T) -> Matrix {
                let n = other.into();
                let mut result = Matrix::new(self.rows, self.cols);

                for r in 0..result.rows {
                    for c in 0..result.cols {
                        result.data[r][c] = self.data[r][c] - n;
                    }
                }

                result
            }
        }

        impl<T: Into<f64>> ops::Mul<T> for $t {
            type Output = Matrix;

            fn mul(self, other: T) -> Matrix {
                let n = other.into();
                let mut result = Matrix::new(self.rows, self.cols);

                for r in 0..result.rows {
                    for c in 0..result.cols {
                        result.data[r][c] = self.data[r][c] * n;
                    }
                }

                result
            }
        }
    };
}

// Matrix-to-matrix operations (non-assigning)
impl_matrix_ops!(Matrix, Matrix);
impl_matrix_ops!(Matrix, &Matrix);
impl_matrix_ops!(&Matrix, Matrix);
impl_matrix_ops!(&Matrix, &Matrix);

// Matrix-to-matrix operations (assigning) and matrix-to-scalar operations
impl_matrix_ops!(Matrix);
impl_matrix_ops!(&Matrix);

impl<T: Into<f64>> ops::AddAssign<T> for Matrix {
    fn add_assign(&mut self, other: T) {
        let n = other.into();
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.data[r][c] += n;
            }
        }
    }
}

impl<T: Into<f64>> ops::SubAssign<T> for Matrix {
    fn sub_assign(&mut self, other: T) {
        let n = other.into();
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.data[r][c] -= n;
            }
        }
    }
}

impl<T: Into<f64>> ops::MulAssign<T> for Matrix {
    fn mul_assign(&mut self, other: T) {
        let n = other.into();

        for r in 0..self.rows {
            for c in 0..self.cols {
                self.data[r][c] *= n;
            }
        }
    }
}

// Matrix from vector of vectors
impl<T: Into<f64> + Copy> From<Vec<Vec<T>>> for Matrix {
    fn from(vec: Vec<Vec<T>>) -> Matrix {
        let first_size = vec[0].len();
        for i in 1..vec.len() {
            if vec[i].len() != first_size {
                panic!("Matrix rows must all be same length");
            }
        }
        let mut result = Matrix::new(vec.len(), first_size);

        for r in 0..result.rows {
            for c in 0..result.cols {
                result.data[r][c] = vec[r][c].into();
            }
        }

        result
    }
}
