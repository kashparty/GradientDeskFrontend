use crate::activation::Activation;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LayerData {
    pub size: usize,
    pub activation: Activation,
}
