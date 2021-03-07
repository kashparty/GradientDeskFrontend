use crate::activation::Activation;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Layer {
    pub size: usize,
    pub activation: Activation,
}
