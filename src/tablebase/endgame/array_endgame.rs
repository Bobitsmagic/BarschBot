use std::{fs, mem};
use std::collections::HashMap;

const RADIX_LENGTH: usize = 20;
const RADIX: usize = 1 << RADIX_LENGTH;
const BIT_MASK: usize = RADIX - 1;

fn calculate_mask(key: u64) -> usize {
    return (key >> (64 - RADIX_LENGTH)) as usize & BIT_MASK;
}

fn compute_jump_table(keys: &Vec<u64>) -> Vec<usize> {
    let mut counter = vec![0 as usize; RADIX];
    for i in 0..keys.len() {
        let mask = calculate_mask(keys[i]);
        
        counter[mask] += 1;
    }

    let mut jump_table = vec![0 as usize; RADIX + 1];
    for r in 1..(RADIX + 1) {
        jump_table[r] = jump_table[r - 1] + counter[r - 1];
    }

    return jump_table;
}

pub struct ArrayEndgame {
    keys: Vec<u64>,
    values: Vec<i16>,
    jump_table: Vec<usize>
}

impl ArrayEndgame {
    pub fn empty() -> ArrayEndgame {
        return ArrayEndgame {
            keys: Vec::new(),
            values: Vec::new(),
            jump_table: vec![0; RADIX + 1]
        };
    }

    pub fn from_hashmap(mut data: HashMap<u64, i16>) -> ArrayEndgame {
        return ArrayEndgame::new(data.drain().collect::<Vec<_>>());
    }
    pub fn len(&self) -> usize {
        return self.keys.len();
    }

    pub fn store_bytes(&self) -> Vec<u8> {
        let mut ret = Vec::with_capacity(10 * self.keys.len());

        assert!(self.keys.is_sorted());
        
        for hash in &self.keys {
            ret.extend(hash.to_be_bytes());
        }

        for eval in &self.values {
            ret.extend(eval.to_be_bytes());
        }

        assert_eq!(ret.len(), 10 * self.keys.len());
       
        return ret;
    }
    
    pub fn store_file(&self) {
        let file_name = format!("data/full_array_table.bin");
        fs::write(file_name, self.store_bytes()).expect("Could not write to file");
    }

    pub fn load_file() -> ArrayEndgame {
        let file_name = format!("data/full_array_table.bin");
        let bytes = fs::read(file_name).expect("Could not read file");

        let length = bytes.len() / 10;

        let mut keys = Vec::with_capacity(length);
        let mut min_index = 0;
        for _ in 0..length {
            keys.push(u64::from_be_bytes(bytes[min_index..min_index + 8].try_into().unwrap()));
            min_index += 8;
        }
        assert!(keys.is_sorted());

        let mut values = Vec::with_capacity(length);
        for _ in 0..length {
            values.push(i16::from_be_bytes(bytes[min_index..min_index + 2].try_into().unwrap()));
            min_index += 2;
        }

        let jump_table = compute_jump_table(&keys);

        return ArrayEndgame {
            keys, values, jump_table
        }
    }

    pub fn new(mut data: Vec<(u64, i16)>) -> ArrayEndgame {
        if !data.is_sorted_by(|(x, _), (y, _)| x <= y) {
            data.sort_unstable_by(|(x, _), (y, _)| x.cmp(y));
        }

        let keys: Vec<u64> = data.iter().map(|x| x.0).collect::<Vec<_>>();
        let values: Vec<i16> = data.iter().map(|x| x.1).collect::<Vec<_>>();
        let jump_table = compute_jump_table(&keys);
        
        return ArrayEndgame {keys, values, jump_table};
    }


    pub fn contains_key(&self, key: u64) -> bool {
        let mask = calculate_mask(key);

        let start = self.jump_table[mask];
        let end = self.jump_table[mask + 1];

        for i in start..end {
            if self.keys[i] == key {
                return true;
            }
        }

        return false;
    }

    pub fn get(&self, key: u64) -> Option<i16> {
        let mask = calculate_mask(key);

        let start = self.jump_table[mask];
        let end = self.jump_table[mask + 1];

        for i in start..end {
            if self.keys[i] == key {
                return Some(self.values[i]);
            }
        }

        return None;
    }   
}