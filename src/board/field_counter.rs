use rand::RngCore;

use crate::board::bit_array::BitArray;


#[derive(Clone, PartialEq, Eq)]
pub struct FieldCounter {
    bits: [u64; 3],
}

impl FieldCounter {
    pub fn empty() -> FieldCounter {
        FieldCounter {
            bits: [0; 3],
        }
    }

    pub fn attacked(&self) -> u64 {
        self.bits[0] | self.bits[1] | self.bits[2]
    }

    pub fn increment(&mut self, mut v1: u64) {
        let carry = v1 & self.bits[0];
        self.bits[0] ^= v1;
        v1 = carry & self.bits[1];
        self.bits[1] ^= carry;

        debug_assert!(v1 & self.bits[2] == 0, "Overflow");

        self.bits[2] ^= v1;

    }

    pub fn decrement(&mut self, mut v1: u64) {
        let carry = v1 & !self.bits[0];
        self.bits[0] ^= v1;
        v1 = carry & !self.bits[1];
        self.bits[1] ^= carry;

        debug_assert!(v1 & !self.bits[2] == 0, "Overflow");

        self.bits[2] ^= v1;
    }

    pub fn count(&self, index: i8) -> u64 {
        let mut value = (self.bits[0] >> index) & 1;
        value |= ((self.bits[1] >> index) & 1) << 1;
        value |= ((self.bits[2] >> index) & 1) << 2;

        return value;
    }
}

#[test] 
fn test_field_increment() {
    let mut counter = FieldCounter::empty();
    let mut field = [0_u64; 64];

    let mut rng = rand::thread_rng();
    for _ in 0..7 {
        let bits = rng.next_u64();

        bits.print();

        counter.increment(bits);

        for j in 0..64 {
            let val = (bits >> j) & 1;
            field[j] += val;
        }

        println!("Field: {:?}", field);

        for j in 0..64 {
            assert_eq!(field[j], counter.count(j as i8));
        }
    }
}

#[test]
fn test_field_decrement() {
    let mut counter = FieldCounter::empty();
    for _ in 0..7 {
        counter.increment(u64::MAX);
    }

    let mut field = [7_u64; 64];

    let mut rng = rand::thread_rng();
    for _ in 0..7 {
        let bits = rng.next_u64();

        bits.print();

        counter.decrement(bits);

        for j in 0..64 {
            let val = (bits >> j) & 1;
            field[j] -= val;
        }

        println!("Field: {:?}", field);

        for j in 0..64 {
            assert_eq!(field[j], counter.count(j as i8));
        }
    }
}
