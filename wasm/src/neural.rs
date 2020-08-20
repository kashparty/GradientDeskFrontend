use crate::matrix::Matrix;
use crate::activation::*;
use crate::layer::LayerData;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ModelData {
    pub layers: Vec<LayerData>,
    pub learning_rate: f64,
    pub momentum_coefficient: f64,
    // TODO: add loss function
}

#[derive(Serialize, Deserialize)]
pub struct TrainingData {
    pub inputs: Matrix,
    pub targets: Matrix,
    pub epochs: usize,
    // TODO: add batch size
    // TODO: add validation split
    // TODO: add metrics
}
#[wasm_bindgen]
pub struct NeuralNetwork {
    layers: Vec<LayerData>,
    weights: Vec<Matrix>,
    weighted_inputs: Vec<Matrix>, // Values of neurons in each layer before activation
    layer_outputs: Vec<Matrix>, // Values of neurons in each layer after activation
    weight_updates: Vec<Matrix>,
    learning_rate: f64,
    momentum: Vec<Matrix>,
    momentum_coefficient: f64,
}

fn squared_error(predictions: &Matrix, targets: Matrix) -> Matrix {
    let mut error = predictions - targets;
    error.map_mut(&|x: f64| x.powi(2));

    error
}

fn squared_error_prime(predictions: &Matrix, targets: Matrix) -> Matrix {
    predictions - targets
}

#[wasm_bindgen]
impl NeuralNetwork {
    #[wasm_bindgen(constructor)]
    pub fn new(model_data: &JsValue) -> NeuralNetwork {
        let model_data: ModelData = model_data.into_serde().unwrap();
        let layers = model_data.layers;
        let learning_rate = model_data.learning_rate;
        let momentum_coefficient = model_data.momentum_coefficient;

        let mut weights: Vec<Matrix> = Vec::new();
        let mut momentum: Vec<Matrix> = Vec::new();
        for i in 0..layers.len() - 1 {
            let mut layer = Matrix::new(layers[i + 1].size, layers[i].size + 1); // Add one neuron as bias in current layer
            momentum.push(layer.clone());
            layer.fill_random();
            weights.push(layer);
        }
        let weighted_inputs = Vec::new();
        let layer_outputs = Vec::new();
        let weight_updates = Vec::new();

        NeuralNetwork { layers, weights, weighted_inputs, layer_outputs, weight_updates, learning_rate, momentum, momentum_coefficient }
    }

    pub fn forward(&mut self, inputs: &JsValue) {
        let inputs: Matrix = inputs.into_serde().unwrap();
        let mut input_matrix = inputs.transpose(); // Reshaped to (number of features, number of samples)

        self.layer_outputs = Vec::new();
        for l in 0..self.layers.len() - 1 {
            input_matrix.add_row(1); // Add bias as input of 1
            self.layer_outputs.push(input_matrix.clone());

            let output_matrix = &self.weights[l] * input_matrix;
            self.weighted_inputs.push(output_matrix.clone());

            input_matrix = self.layers[l].activation.f(&output_matrix);
        }
        self.layer_outputs.push(input_matrix.clone());
    }

    pub fn predict(&mut self, inputs: &JsValue) -> JsValue {
        self.forward(inputs);
        JsValue::from_serde(&self.layer_outputs[self.layer_outputs.len() - 1]).unwrap()
    }

    pub fn calculate_error(&mut self, targets: &JsValue) -> f64 {
        let targets: Matrix = targets.into_serde().unwrap();
        let target_matrix = targets.transpose();

        let error_matrix = squared_error(&self.layer_outputs[self.layer_outputs.len() - 1], target_matrix);
        let error = error_matrix.get_data();
        let mut sum = 0.0;
        for r in 0..error.len() {
            for c in 0..error[0].len() {
                sum += error[r][c];
            }
        }
        let mse = sum / error[0].len() as f64;
        mse
    }

    pub fn backprop(&mut self, targets: &JsValue) {
        let targets: Matrix = targets.into_serde().unwrap();
        let target_matrix = targets.transpose(); // Reshaped to (number of output nodes, number of samples)

        // Calculate output error
        let mut chain = squared_error_prime(&self.layer_outputs[self.layer_outputs.len() - 1], target_matrix);
        chain.element_wise_multiply_mut(
            self.layers[self.layers.len() - 1].activation.f_prime(
                &self.weighted_inputs[self.layer_outputs.len() - 2]
            )
        );

        self.weight_updates = Vec::new();
        self.weight_updates.push(&chain * self.layer_outputs[self.layer_outputs.len() - 2].transpose());

        for l in (1..self.layer_outputs.len() - 1).rev() {
            // Iterate through layers backwards
            chain = self.weights[l].transpose() * chain;
            chain.remove_row();
            chain.element_wise_multiply_mut(self.layers[l - 1].activation.f_prime(&self.weighted_inputs[l - 1]));
            self.weight_updates.push(&chain * self.layer_outputs[l - 1].transpose());
        }

        self.weight_updates.reverse();
    }

    pub fn update_weights(&mut self, training_sample_size: usize) {
        for u in 0..self.weights.len() {
            self.momentum[u] *= self.momentum_coefficient;
            self.momentum[u] -= &self.weight_updates[u] * (self.learning_rate / training_sample_size as f64);
            self.weights[u] += &self.momentum[u];
        }
    }
}
