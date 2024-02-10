use std::os::windows;
use rand::prelude::*;
use rand_distr::StandardNormal;

pub struct Perceptron {
    pub weights: Vec<i32>
}

pub fn test_perceptron() {
    const DIMENSIONS: usize = 6;
    const SAMPLE_COUNT: usize = 12_000_00;
    let mut rng = thread_rng();

    let target_weights = vec![1, 3, 7, 42, -69, 99];

    let teacher = Perceptron { weights: target_weights };

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for i in 0..SAMPLE_COUNT {
        let mut vec: Vec<i32> = Vec::new();
        for d in 0..DIMENSIONS {
            vec.push(rng.gen_range(-100..101));
        }
        
        let str: i32 =  rng.gen_range(-10..11);
        outputs.push(teacher.calc_output(&vec) + str);
        inputs.push(vec);
    }

    let mut nn = Perceptron::new(DIMENSIONS);
    nn.randomize_weights();

    println!("Started gdc");
    nn.gradient_descent(&inputs, &outputs);
}

impl Perceptron {
    pub fn new(weight_count: usize) -> Self {
        return Perceptron { weights: vec![0; weight_count]  }
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
            self.weights[i] = rng.gen_range(-100..101);
        }
    }

    pub fn calc_squares_error(&self, input_set: &Vec<Vec<i32>>, output_set: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for input_index in 0..input_set.len() {
            let error = output_set[input_index] - self.calc_output(&input_set[input_index]);

            sum += error * error;
        }

        return sum;
    }

    pub fn calc_output(&self, input: &Vec<i32>) -> i32 {
        assert_eq!(input.len(), self.weights.len());

        let mut sum = 0;
        for i in 0..input.len() {
            sum += input[i] * self.weights[i];
        }

        return sum;
    }

    pub fn calc_gradient(&self, input_set: &Vec<Vec<i32>>, output_set: &Vec<i32>) -> Vec<i32> {
        let mut gradient = vec![0; input_set[0].len()];

        for input_index in 0..input_set.len() {
            let error = output_set[input_index] - self.calc_output(&input_set[input_index]);

            for w in 0..gradient.len() {
                gradient[w] += input_set[input_index][w] * error;
            }
        }

        return gradient;
    }

    pub fn calc_stochastic_gradient(&self, input_set: &Vec<Vec<i32>>, output_set: &Vec<i32>, index_set: &Vec<usize>) -> Vec<i32> {
        let mut gradient = vec![0; input_set[0].len()];


        for input_index in index_set {
            let error = output_set[*input_index] - self.calc_output(&input_set[*input_index]);

            for w in 0..gradient.len() {
                gradient[w] += input_set[*input_index][w] * error;
            }
        }

        return gradient;
    }

    pub fn gradient_descent(&mut self, input_set: &Vec<Vec<i32>>, output_set: &Vec<i32>) {

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
                self.weights[i] += (gradient[i] / SAMPLE_COUNT as i32).signum();
            }
            if i % 10000 == 0 {
                println!("Sq");
                println!("{} -> {}", i, (self.calc_squares_error(input_set, output_set) / input_set.len() as i32));
                self.print();
            }
        }
        println!("NSE: {}", self.calc_squares_error(input_set, output_set) / input_set.len() as i32);
    }
}
