use crate::layer::Layer;
use crate::matrix::Matrix;
use crate::loss::*;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NesterovParams {
    pub layers: Vec<Layer>,
    pub loss: Loss,
    pub learning_rate: f64,
    pub momentum_coefficient: f64,
}

#[wasm_bindgen]
pub struct Nesterov {
    layers: Vec<Layer>,
    loss: Loss,
    weights: Vec<Matrix>,
    weighted_inputs: Vec<Matrix>, // Values of nodes in each layer before activation
    layer_outputs: Vec<Matrix>, // Values of nodes in each layer after activation
    weight_updates: Vec<Matrix>,
    learning_rate: f64,
    momentum: Vec<Matrix>,
    temp_momentum: Vec<Matrix>,
    momentum_coefficient: f64
}

#[wasm_bindgen]
impl Nesterov {
    #[wasm_bindgen(constructor)]
    pub fn new(params: &JsValue) -> Nesterov {
        let params: NesterovParams = params.into_serde().unwrap();
        let mut weights: Vec<Matrix> = Vec::new();
        let mut momentum: Vec<Matrix> = Vec::new();
        let mut temp_momentum: Vec<Matrix> = Vec::new();
        for i in 0..params.layers.len() - 1 {
            let mut layer = Matrix::new(params.layers[i + 1].size, params.layers[i].size + 1); // Add one neuron as bias in current layer
            momentum.push(layer.clone());
            layer.fill_random();
            temp_momentum.push(layer.clone());
            weights.push(layer);
        }
        let weighted_inputs = Vec::new();
        let layer_outputs = Vec::new();
        let weight_updates = Vec::new();

        Self { 
            layers: params.layers,
            loss: params.loss, 
            weights, 
            weighted_inputs, 
            layer_outputs, 
            weight_updates, 
            learning_rate: params.learning_rate,
            momentum,
            temp_momentum,
            momentum_coefficient: params.momentum_coefficient,
        }
    }

    pub fn forward(&mut self, inputs: &JsValue) {
        let inputs: Matrix = inputs.into_serde().unwrap();
        let mut input_matrix = inputs.transpose(); // Reshaped to (number of features, number of samples)

        for u in 0..self.weights.len() {
            self.temp_momentum[u] = &self.weights[u] - &self.momentum[u] * self.momentum_coefficient; 
        }

        self.layer_outputs = Vec::new();
        for l in 0..self.layers.len() - 1 {
            input_matrix.add_row(1); // Add bias as input of 1
            self.layer_outputs.push(input_matrix.clone());

            let output_matrix = &self.weights[l] * input_matrix;
            self.weighted_inputs.push(output_matrix.clone());

            input_matrix = self.layers[l + 1].activation.f(&output_matrix);
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

        self.loss.error(&self.layer_outputs[self.layer_outputs.len() - 1], &target_matrix)
    }

    pub fn backprop(&mut self, targets: &JsValue) {
        let targets: Matrix = targets.into_serde().unwrap();
        let target_matrix = targets.transpose(); // Reshaped to (number of output nodes, number of samples)

        // Calculate output error
        let mut chain = self.loss.error_prime(&self.layer_outputs[self.layer_outputs.len() - 1], &target_matrix);
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

    pub fn update_weights(&mut self) {
        for u in 0..self.weights.len() {
            self.momentum[u] *= self.momentum_coefficient;
            self.momentum[u] += &self.weight_updates[u] * self.learning_rate;
            self.weights[u] -= &self.momentum[u];
        }
    }

    pub fn get_weights(&self) -> JsValue {
        JsValue::from_serde(&self.weights).unwrap()
    }
}