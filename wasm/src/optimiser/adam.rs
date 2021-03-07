use crate::layer::Layer;
use crate::loss::*;
use crate::matrix::Matrix;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AdamParams {
    pub layers: Vec<Layer>,
    pub loss: Loss,
    pub learning_rate: f64,
    pub beta1: f64,
    pub beta2: f64,
}

#[wasm_bindgen]
pub struct Adam {
    layers: Vec<Layer>,
    loss: Loss,
    weights: Vec<Matrix>,
    weighted_inputs: Vec<Matrix>, // Values of nodes in each layer before activation
    layer_outputs: Vec<Matrix>,   // Values of nodes in each layer after activation
    weight_updates: Vec<Matrix>,
    weighted_averages: Vec<Matrix>,
    weighted_squared_averages: Vec<Matrix>,
    learning_rate: f64,
    beta1: f64,
    beta2: f64,
    t: i32,
}

#[wasm_bindgen]
impl Adam {
    #[wasm_bindgen(constructor)]
    pub fn new(params: &JsValue) -> Adam {
        let params: AdamParams = params.into_serde().unwrap();

        let mut weights: Vec<Matrix> = Vec::new();
        let mut weighted_averages: Vec<Matrix> = Vec::new();
        let mut weighted_squared_averages: Vec<Matrix> = Vec::new();
        for i in 0..params.layers.len() - 1 {
            let mut layer = Matrix::new(params.layers[i + 1].size, params.layers[i].size + 1); // Add one neuron as bias in current layer
            weighted_averages.push(layer.clone());
            weighted_squared_averages.push(layer.clone());
            layer.fill_random();
            weights.push(layer);
        }
        let weighted_inputs = Vec::new();
        let layer_outputs = Vec::new();
        let weight_updates = Vec::new();

        Adam {
            layers: params.layers,
            loss: params.loss,
            weights,
            weighted_inputs,
            layer_outputs,
            weight_updates,
            weighted_averages,
            weighted_squared_averages,
            learning_rate: params.learning_rate,
            beta1: params.beta1,
            beta2: params.beta2,
            t: 0,
        }
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

        self.loss.error(
            &self.layer_outputs[self.layer_outputs.len() - 1],
            &target_matrix,
        )
    }

    pub fn backprop(&mut self, targets: &JsValue) {
        let targets: Matrix = targets.into_serde().unwrap();
        let target_matrix = targets.transpose(); // Reshaped to (number of output nodes, number of samples)

        // Calculate output error
        let mut chain = self.loss.error_prime(
            &self.layer_outputs[self.layer_outputs.len() - 1],
            &target_matrix,
        );
        chain.element_wise_multiply_mut(
            self.layers[self.layers.len() - 1]
                .activation
                .f_prime(&self.weighted_inputs[self.layer_outputs.len() - 2]),
        );

        self.weight_updates = Vec::new();
        self.weight_updates
            .push(&chain * self.layer_outputs[self.layer_outputs.len() - 2].transpose());

        for l in (1..self.layer_outputs.len() - 1).rev() {
            // Iterate through layers backwards
            chain = self.weights[l].transpose() * chain;
            chain.remove_row();
            chain.element_wise_multiply_mut(
                self.layers[l - 1]
                    .activation
                    .f_prime(&self.weighted_inputs[l - 1]),
            );
            self.weight_updates
                .push(&chain * self.layer_outputs[l - 1].transpose());
        }

        self.weight_updates.reverse();
    }

    pub fn update_weights(&mut self) {
        // Adam algorithm
        self.t += 1;
        for u in 0..self.weights.len() {
            // m = beta1 * m + (1 - beta1) * g
            self.weighted_averages[u] = &self.weighted_averages[u] * self.beta1
                + &self.weight_updates[u] * (1.0 - self.beta1);

            // v = beta2 * v + (1 - beta2) * g^2
            self.weighted_squared_averages[u] = &self.weighted_squared_averages[u] * self.beta2
                + &self.weight_updates[u].map(&|x| x * x) * (1.0 - self.beta2);

            // println!("{}", self.weighted_squared_averages[u]);

            // M = m / (1 - beta1^t)
            let corrected_weighted_averages =
                &self.weighted_averages[u] * (1.0 / (1.0 - self.beta1.powi(self.t)));

            // V = v / (1 - beta2^t)
            let corrected_weighted_squared_averages =
                &self.weighted_squared_averages[u] * (1.0 / (1.0 - self.beta2.powi(self.t)));

            // theta = theta - a * M / (sqrt(V) + epsilon)
            let mut denominator = corrected_weighted_squared_averages.clone();
            denominator.map_mut(&|x| 1.0 / (1e-8 + x.sqrt()));

            // Subtract the gradients for each weight matrix
            self.weights[u] -=
                denominator.element_wise_multiply(corrected_weighted_averages) * self.learning_rate
        }
    }

    pub fn get_weights(&self) -> JsValue {
        JsValue::from_serde(&self.weights).unwrap()
    }
}
