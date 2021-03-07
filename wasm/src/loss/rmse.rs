use crate::matrix::Matrix;
use super::MeanSquaredError;

pub struct RootMeanSquaredError {}

impl RootMeanSquaredError {
    pub fn error(predictions: &Matrix, targets: &Matrix) -> f64 {
        MeanSquaredError::error(predictions, targets).sqrt()
    }

    pub fn error_prime(predictions: &Matrix, targets: &Matrix) -> Matrix {
        let denominator = 1.0 / (2.0 * MeanSquaredError::error(predictions, targets).sqrt());
        let numerator = MeanSquaredError::error_prime(predictions, targets);
        numerator * denominator
    }
}