use crate::matrix::Matrix;

pub struct CategoricalCrossentropy {}

impl CategoricalCrossentropy {
    pub fn error(predictions: &Matrix, targets: &Matrix) -> f64 {
        // Softmax
        let expo = predictions.map(&|x: f64| x.exp());
        let mut expo_data = expo.get_data().clone();
        let mut totals: Vec<f64> = Vec::new();
        for c in 0..expo_data[0].len() {
            let mut total = 0.0;
            for r in 0..expo_data.len() {
                total += expo_data[r][c];
            }
            totals.push(total);
        }

        for c in 0..expo_data[0].len() {
            for r in 0..expo_data.len() {
                expo_data[r][c] /= totals[c];
            }
        }
        
        // Cross-entropy
        let softmax: Matrix = expo_data.into();
        softmax.map(&|x: f64| -(x.log2()));
        let error = targets.element_wise_multiply(softmax);

        let error_data = error.get_data();
        let mut sum = 0.0;
        for r in 0..error_data.len() {
            for c in 0..error_data[0].len() {
                sum += error_data[r][c];
            }
        }
        let error = sum / error_data[0].len() as f64;
        error
    }

    pub fn error_prime(predictions: &Matrix, targets: &Matrix) -> Matrix {
        // Softmax
        let expo = predictions.map(&|x: f64| x.exp());
        let mut expo_data = expo.get_data().clone();
        let mut totals: Vec<f64> = Vec::new();
        for c in 0..expo_data[0].len() {
            let mut total = 0.0;
            for r in 0..expo_data.len() {
                total += expo_data[r][c];
            }
            totals.push(total);
        }

        for c in 0..expo_data[0].len() {
            for r in 0..expo_data.len() {
                expo_data[r][c] /= totals[c];
            }
        }

        let softmax: Matrix = expo_data.into();
        softmax - targets
    }
}