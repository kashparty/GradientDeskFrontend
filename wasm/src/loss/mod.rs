mod mae;
mod mse;
mod rmse;
mod crossentropy;

use crate::matrix::Matrix;
pub use mae::MeanAbsoluteError;
pub use mse::MeanSquaredError;
pub use rmse::RootMeanSquaredError;
pub use crossentropy::CategoricalCrossentropy;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Loss {
    MeanSquaredError,
    MeanAbsoluteError,
    RootMeanSquaredError,
    CategoricalCrossentropy
}

impl Loss {
    pub fn error(self, predictions: &Matrix, targets: &Matrix) -> f64 {
        match self {
            Self::MeanSquaredError => MeanSquaredError::error(predictions, targets),
            Self::MeanAbsoluteError => MeanAbsoluteError::error(predictions, targets),
            Self::RootMeanSquaredError => RootMeanSquaredError::error(predictions, targets),
            Self::CategoricalCrossentropy => CategoricalCrossentropy::error(predictions, targets),
        }
    }

    pub fn error_prime(self, predictions: &Matrix, targets: &Matrix) -> Matrix {
        match self {
            Self::MeanSquaredError => MeanSquaredError::error_prime(predictions, targets),
            Self::MeanAbsoluteError => MeanAbsoluteError::error_prime(predictions, targets),
            Self::RootMeanSquaredError => RootMeanSquaredError::error_prime(predictions, targets),
            Self::CategoricalCrossentropy => CategoricalCrossentropy::error_prime(predictions, targets),
        }
    }
}