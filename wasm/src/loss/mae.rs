use crate::matrix::Matrix;

pub struct MeanAbsoluteError {}

impl MeanAbsoluteError {
    pub fn error(predictions: &Matrix, targets: &Matrix) -> f64 {
        let mut error = predictions - targets;
        error.map_mut(&|x: f64| x.abs());

        let error_data = error.get_data();
        let mut sum = 0.0;
        for r in 0..error_data.len() {
            for c in 0..error_data[0].len() {
                sum += error_data[r][c];
            }
        }
        let mae = sum / error_data[0].len() as f64;
        mae
    }

    pub fn error_prime(predictions: &Matrix, targets: &Matrix) -> Matrix {
        (predictions - targets).map(&|x: f64| x.signum())
    }
}