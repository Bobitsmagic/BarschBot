use crate::evaluation::{generate_eval_attributes, generate_eval_attributes_fast, static_eval_float};
use crate::perceptron_float;
use crate::{bit_board::BitBoard, barsch_bot, game::Game, perceptron_float::Perceptron};
use std::cmp;
use std::fs::{read_to_string, self};
use rand::{thread_rng, Rng};

pub struct EvalBoards {
    boards: Vec<BitBoard>,
    evals: Vec<i32>
}

impl EvalBoards {
    pub fn load(path: &str) -> EvalBoards {
        println!("Loading dataset at {}", path);

        let mut boards = Vec::new();
        let mut evals = Vec::new();

        let mut positions = 0;
        for line in read_to_string(path).unwrap().lines().skip(1) {
            let parts = line.split(",").collect::<Vec<_>>();
            
            positions += 1;

            //skipping all checkmates for now
            if parts[1].contains('#') {
                continue;
            }

            let mut board = BitBoard::from_fen(parts[0]);
            
            if !barsch_bot::is_quiet_pos(&mut board) {
                continue;
            }

            let mut skip = 0;
            for c in parts[1].chars() {
                if c == '+' || c == '-' || c == '0' {
                    break;
                }

                skip += 1;
            }


            let val_string: String = parts[1].chars().skip(skip).collect();
            let opt = val_string.parse();
            if opt.is_err() {
                println!("Error at {}[{}]", val_string.len(), val_string);
            }
            
            let eval: i32 = opt.unwrap();

            if eval.abs() > 100 {
                //continue;
            }

            boards.push(board);
            evals.push(eval * 10);

            if positions % 1_000_000 == 0{
                println!("{}", positions);
            }   
        }

        println!("Totol position count: {}, filtered count: {}", positions, boards.len());

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for i in &evals {
            min = i32::min(min, *i);
            max = i32::max(max, *i);
        }

        for i in 0..evals.len() {
            if evals[i] == min || evals[i] == max {
                boards[i].print();
                
                println!("Eval: {}", evals[i]);
                break;
            }
        }

        println!("Min eval: {} Max eval {}", min, max);

        return EvalBoards { boards, evals };
    } 

    pub fn create_input_set(&self) -> Vec<Vec<f32>> {
        let mut ret = Vec::new();
        println!("Creating input set");
        for i in 0..self.boards.len() {

            ret.push(generate_eval_attributes(&self.boards[i]).get_vector());

            if i % 1_000_000 == 0 {
                println!("{} -> {}%", i, (i as f64 / self.boards.len() as f64) * 100.0);
            }  
        }

        return ret;
    }

    pub fn create_input_set_int(&self) -> Vec<Vec<f64>> {
        let mut ret = Vec::new();
        println!("Creating input set");
        for i in 0..self.boards.len() {

            let (array, mat_sum) = generate_eval_attributes_fast(&self.boards[i]).get_vector();

            let mut vec = Vec::new();

            let scale = mat_sum as f64 / 24.0;
            for i in 0..array.len() {
                vec.push(array[i] as f64 * scale);
            }

            let scale = (24 - mat_sum) as f64 / 24.0;
            for i in 0..array.len() {
                vec.push(array[i] as f64 * scale);
            }

            ret.push(vec);

            if i % 1_000_000 == 0 {
                println!("{} -> {}%", i, (i as f64 / self.boards.len() as f64) * 100.0);
            }  
        }

        return ret;
    }

    pub fn create_output_set(&self) -> Vec<i32> {
        return self.evals.clone();

        //fn normalize(val: f64) -> f64 {
        //    return f64::max(f64::min(val, 10_000.0), -10_000.0) / 10_000.0;
        //}
    }
}