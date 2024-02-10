use std::{ops::Mul, time::Instant};

use na::{ComplexField, DMatrix};
use opengl_graphics::error;
use rand::prelude::*;
use rand_distr::StandardNormal;
use nalgebra as na;

pub struct Perceptron {
    pub weights: Vec<f64>
}

pub fn test_gauss_newton() {
    let mut m = na::DMatrix::from_element(2, 2, 0.0);
    m[(0, 0)] = 1.0;
    m[(0, 1)] = 2.0;
    m[(1, 0)] = 3.0;
    m[(1, 1)] = 4.0;

    match m.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {}", inv);
        }
        None => {
            println!("m1 is not invertible!");
        }
    }

}




pub fn test_perceptron() {
    const DIMENSIONS: usize = 30;
    const SAMPLE_COUNT: usize = 12_000_000;
    let mut rng = thread_rng();

    //let target_weights = vec![3.14159274, -2.71828175, -42.0, 1337.0, 69.0, -31.0];
    let mut target_weights: Vec<f64> = vec![0.0; DIMENSIONS];
    for i in 0..DIMENSIONS {
        target_weights[i] = rng.sample(StandardNormal);
    }

    let teacher = Perceptron { weights: target_weights };

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for i in 0..SAMPLE_COUNT {
        let mut vec: Vec<f64> = Vec::new();
        for d in 0..DIMENSIONS {
            vec.push(rng.sample(StandardNormal));
        }
        
        let str: f64 =  rng.sample(StandardNormal);
        outputs.push(teacher.calc_output(&vec) + str * 10.0);
        inputs.push(vec);
    }

    let mut nn = Perceptron::new(DIMENSIONS);
    nn.randomize_weights();

    println!("Started gdc");
    //nn.gradient_descent(&inputs, &outputs);
    nn.gauss_newton(&inputs, &outputs);
}

impl Perceptron {
    pub fn new(weight_count: usize) -> Self {
        return Perceptron { weights: vec![0.0; weight_count]  }
    }

    pub fn print(&self) {
        println!("Weights: ");
        for w in &self.weights {
            print!("{} ", w);
        }
        println!();
    }

    pub fn randomize_weights(&mut self) {
        let mut rng = thread_rng();

        for i in 0..self.weights.len() {
            self.weights[i] = rng.sample(StandardNormal);
        }
    }

    pub fn calc_squares_error(&self, input_set: &Vec<Vec<f64>>, output_set: &Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for input_index in 0..input_set.len() {
            let error = output_set[input_index] - self.calc_output(&input_set[input_index]);

            sum += error * error;
        }

        return sum;
    }

    pub fn calc_output(&self, input: &Vec<f64>) -> f64 {
        assert_eq!(input.len(), self.weights.len());

        let mut sum = 0.0;
        for i in 0..input.len() {
            sum += input[i] * self.weights[i];
        }

        return sum;
    }

    pub fn calc_gradient(&self, input_set: &Vec<Vec<f64>>, output_set: &Vec<f64>) -> Vec<f64> {
        let mut gradient = vec![0.0; input_set[0].len()];

        for input_index in 0..input_set.len() {
            let error = output_set[input_index] - self.calc_output(&input_set[input_index]);

            for w in 0..gradient.len() {
                gradient[w] += input_set[input_index][w] * error;
            }
        }

        return gradient;
    }

    pub fn calc_stochastic_gradient(&self, input_set: &Vec<Vec<f64>>, output_set: &Vec<f64>, index_set: &Vec<usize>) -> Vec<f64> {
        let mut gradient = vec![0.0; input_set[0].len()];


        for input_index in index_set {
            let error = output_set[*input_index] - self.calc_output(&input_set[*input_index]);

            for w in 0..gradient.len() {
                gradient[w] += input_set[*input_index][w] * error;
            }
        }

        return gradient;
    }

    pub fn gradient_descent(&mut self, input_set: &Vec<Vec<f64>>, output_set: &Vec<f64>) {

        const MAX_IT: usize = 100_0000;
        const SAMPLE_COUNT: usize = 100_00;
        let mut indices: Vec<usize> = Vec::new();
        let mut rng = thread_rng();

        for i in 0..MAX_IT {
            indices.clear();
            for _ in 0..SAMPLE_COUNT {
                indices.push(rng.gen_range(0..SAMPLE_COUNT));
            }

            let gradient = self.calc_stochastic_gradient(input_set, output_set, &indices);
            
            //println!("Gradient");
            //println!("{:?}", gradient);

            //Skip pawn
            for i in 0..gradient.len() {
                self.weights[i] += (gradient[i] / SAMPLE_COUNT as f64) * 0.001;
            }
            if i % 10000 == 0 {
                println!("Sq");
                println!("{} -> {}", i, f64::sqrt(self.calc_squares_error(input_set, output_set) / input_set.len() as f64));
                self.print();
            }
        }
        println!("NSE: {}", self.calc_squares_error(input_set, output_set) / input_set.len() as f64);
    }
    
    pub fn gauss_newton(&mut self, input_set: &Vec<Vec<f64>>, output_set: &Vec<f64>) {
        let S = input_set.len();
        let P = input_set[0].len();

        println!("Initial values");        
        self.print();

        let start = Instant::now();
        let mut hesse = DMatrix::from_element(P, P, 0.0);

        for s in 0..S {
            let set = &input_set[s];
            for p1 in 0..P {
                for p2 in 0..P {
                    hesse[(p1, p2)] += set[p1] * set[p2];
                }
            }
        }
        
        println!("Hesse time: {}", start.elapsed().as_millis());
        //println!("Hesse: {}", hesse);

        
        match hesse.try_inverse() {
            Some(inv) => {
                println!("Inverse time: {}", start.elapsed().as_millis());
                //println!("The inverse of hesse is: {}", inv);
                for i in 0..500 {
                    let error_vector: Vec<f64> = (0..S).map(|s| (self.calc_output(&input_set[s]) - output_set[s])).collect();
                    let mut gradient = na::DVector::from_element(P, 0.0);
                    for p in 0..gradient.len() {
                        let mut sum = 0.0;
                        
                        for s in 0..S {
                            sum += error_vector[s] * input_set[s][p];
                        }
            
                        gradient[p] = sum;
                    }
        
                    let delta = &inv * gradient;
        
                    for p in 0..P {
                        self.weights[p] -= delta[p];
                    }

                    println!("{} -> {}", i, f64::sqrt(self.calc_squares_error(input_set, output_set) / input_set.len() as f64));
                    self.print();
                }
            }
            None => {
                println!("m1 is not invertible!");
            }
        }        
    }
}
