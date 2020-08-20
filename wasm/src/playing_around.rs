mod matrix;
mod neural;
mod activation;
mod layer;

use matrix::Matrix;
use neural::NeuralNetwork;
use layer::LayerData;
use activation::Activation;
//use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut layers: Vec<LayerData> = Vec::new();
    layers.push(LayerData { size: 2, activation: Activation::TanH });
    layers.push(LayerData { size: 5, activation: Activation::TanH });
    layers.push(LayerData { size: 1, activation: Activation::ReLU });
    let mut nn = NeuralNetwork::new(layers, 0.3, 0.9);

    let mut inputs: Vec<Vec<f64>> = Vec::new();
    let mut outputs: Vec<Vec<f64>> = Vec::new();

    //let mut rng = rand::thread_rng();
    //let dist = Uniform::from(-1.0..1.0);

    for _ in 0..1 {
        //let a = dist.sample(&mut rng);
        //let b = dist.sample(&mut rng);
        //inputs.push(vec![a, b]);
        //outputs.push(vec![a * b]);
        
        inputs.push(vec![0.0, 0.0]);
        outputs.push(vec![0.0]);

        inputs.push(vec![0.0, 1.0]);
        outputs.push(vec![1.0]);

        inputs.push(vec![1.0, 0.0]);
        outputs.push(vec![1.0]);

        inputs.push(vec![1.0, 1.0]);
        outputs.push(vec![0.0]);
    }

    let input_matrix: Matrix = inputs.into();
    let output_matrix: Matrix = outputs.into();

    let mut _mse = 10.0;

    for _ in 0..5000 {
        nn.forward(&input_matrix);
        nn.backprop(&output_matrix);
        _mse = nn.calculate_error(&output_matrix);
        nn.update_weights(output_matrix.rows());
    }

    nn.calculate_error(&output_matrix);
    nn.predict(&vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 0.0], vec![1.0, 1.0]].into());
}

