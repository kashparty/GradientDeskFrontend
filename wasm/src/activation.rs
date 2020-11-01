use crate::matrix::Matrix;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Activation {
    TanH,
    ReLU,
    LeakyReLU,
    Sigmoid,
    Linear
}

impl Activation {
    pub fn f(&self, input: &Matrix) -> Matrix {
        match self {
            Self::TanH => TanH::f(input),
            Self::ReLU => ReLU::f(input),
            Self::LeakyReLU => LeakyReLU::f(input),
            Self::Sigmoid => Sigmoid::f(input),
            Self::Linear => Linear::f(input),
        }
    }

    pub fn f_prime(&self, input: &Matrix) -> Matrix {
        match self {
            Self::TanH => TanH::f_prime(input),
            Self::ReLU => ReLU::f_prime(input),
            Self::LeakyReLU => LeakyReLU::f_prime(input),
            Self::Sigmoid => Sigmoid::f_prime(input),
            Self::Linear =>Linear::f_prime(input),
        }
    }
}

pub struct TanH {}

impl TanH {
    fn f(input: &Matrix) -> Matrix {
        input.map(&|x: f64| x.tanh())
    }

    fn f_prime(input: &Matrix) -> Matrix {
        input.map(&|x: f64| 1.0 - x.tanh().powi(2))
    }
}

pub struct ReLU {}

impl ReLU {
    fn f(input: &Matrix) -> Matrix {
        input.map(&|x: f64| {
            if x > 0.0 { x } else { 0.0 }
        })
    }

    fn f_prime(input: &Matrix) -> Matrix {
        input.map(&|x: f64| {
            if x > 0.0 { 1.0 } else { 0.0 }
        })
    }
}

pub struct LeakyReLU {}

impl LeakyReLU {
    fn f(input: &Matrix) -> Matrix {
        input.map(&|x: f64| {
            if x > 0.0 { x } else { 0.01 * x }
        })
    }

    fn f_prime(input: &Matrix) -> Matrix {
        input.map(&|x: f64| {
            if x > 0.0 { 1.0 } else { 0.01 }
        })
    }
}

pub struct Sigmoid {}

impl Sigmoid {
    fn f(input: &Matrix) -> Matrix {
        input.map(&|x: f64| 1.0 / (1.0 + (-x).exp()))
    } 

    fn f_prime(input: &Matrix) -> Matrix {
        Sigmoid::f(input).element_wise_multiply((Sigmoid::f(input) * -1.0) + 1.0)
    }
}

pub struct Linear {}

impl Linear {
    fn f(input: &Matrix) -> Matrix {
        input.clone()
    }

    fn f_prime(input: &Matrix) -> Matrix {
        let mut result = Matrix::new(input.rows(), input.cols());
        result.fill(1);
        result
    }
}