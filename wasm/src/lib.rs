pub mod activation;
pub mod layer;
pub mod matrix;
pub mod loss;
pub mod optimiser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn initialise() {
    console_error_panic_hook::set_once();
}