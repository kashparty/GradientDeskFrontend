pub mod activation;
pub mod layer;
pub mod matrix;
pub mod neural;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use layer::LayerData;
use neural::NeuralNetwork;
use matrix::Matrix;

#[derive(Serialize, Deserialize)]
pub struct TestingData {
    pub inputs: Matrix,
    pub targets: Matrix,
}

#[wasm_bindgen]
pub fn initialise() {
    console_error_panic_hook::set_once();
}