use crate::board::bit_array_lookup::{LEFT_MOVE_MASK, RIGHT_MOVE_MASK};

pub const DIAGONAL_FILL_FUNCTIONS_CAP: [fn(u64, u64) -> u64; 4] = [
    fill_up_right_cap,
    fill_up_left_cap,
    fill_down_left_cap,
    fill_down_right_cap,
];

pub const DIAGONAL_FILL_FUNCTIONS: [fn(u64, u64) -> u64; 4] = [
    fill_up_right,
    fill_up_left,
    fill_down_left,
    fill_down_right,
];

pub const ORTHOGONAL_FILL_FUNCTIONS_CAP: [fn(u64, u64) -> u64; 4] = [
    fill_up_cap,
    fill_left_cap,
    fill_down_cap,
    fill_right_cap,
];

pub const ORTHOGONAL_FILL_FUNCTIONS: [fn(u64, u64) -> u64; 4] = [
    fill_up,
    fill_left,
    fill_down,
    fill_right,
];

// https://www.chessprogramming.org/Kogge-Stone_Algorithm
pub fn fill_up(mut gen: u64, mut free: u64) -> u64 {
    gen |= free & (gen <<  8);
    free &=       free <<  8;
    gen |= free & (gen << 16);
    free &=       free << 16;
    gen |= free & (gen << 32);

    return gen;
}
pub fn fill_up_cap(mut gen: u64, mut free: u64) -> u64 {
    gen |= free & (gen <<  8);
    free &=       free <<  8;
    gen |= free & (gen << 16);
    free &=       free << 16;
    gen |= free & (gen << 32);

    return gen & free | (gen << 8);
}

pub fn fill_down(mut gen: u64, mut free: u64) -> u64 {
    gen |= free & (gen >>  8);
    free &=       free >>  8;
    gen |= free & (gen >> 16);
    free &=       free >> 16;
    gen |= free & (gen >> 32);

    return gen;
}
pub fn fill_down_cap(mut gen: u64, mut free: u64) -> u64 {
    gen |= free & (gen >>  8);
    free &=       free >>  8;
    gen |= free & (gen >> 16);
    free &=       free >> 16;
    gen |= free & (gen >> 32);

    return gen & free | (gen >> 8);
}

//Right moving
pub fn fill_right(mut gen: u64, mut free: u64) -> u64 {
    free &= RIGHT_MOVE_MASK[1];
    gen |= free & (gen << 1);
    free &=       free << 1;
    gen |= free & (gen << 2);
    free &=       free << 2;
    gen |= free & (gen << 4);

    return gen;
}
pub fn fill_right_cap(mut gen: u64, mut free: u64) -> u64 {
    free &= RIGHT_MOVE_MASK[1];
    gen |= free & (gen << 1);
    free &=       free << 1;
    gen |= free & (gen << 2);
    free &=       free << 2;
    gen |= free & (gen << 4);

    return gen & free | (gen << 1) & RIGHT_MOVE_MASK[1];
}

pub fn fill_up_right(mut gen: u64, mut free: u64) -> u64 {
    free &= RIGHT_MOVE_MASK[1];
    gen |= free & (gen <<  9);
    free &=       free <<  9;
    gen |= free & (gen << 18);
    free &=       free << 18;
    gen |= free & (gen << 36);

    return gen;
}
pub fn fill_up_right_cap(mut gen: u64, mut free: u64) -> u64 {
    free &= RIGHT_MOVE_MASK[1];
    gen |= free & (gen <<  9);
    free &=       free <<  9;
    gen |= free & (gen << 18);
    free &=       free << 18;
    gen |= free & (gen << 36);

    return gen & free | (gen << 9) & RIGHT_MOVE_MASK[1];
}

pub fn fill_down_right(mut gen: u64, mut free: u64) -> u64 {
    free &= RIGHT_MOVE_MASK[1];
    gen |= free & (gen >>  7);
    free &=       free >>  7;
    gen |= free & (gen >> 14);
    free &=       free >> 14;
    gen |= free & (gen >> 28);
    return gen;
}
pub fn fill_down_right_cap(mut gen: u64, mut free: u64) -> u64 {
    free &= RIGHT_MOVE_MASK[1];
    gen |= free & (gen >>  7);
    free &=       free >>  7;
    gen |= free & (gen >> 14);
    free &=       free >> 14;
    gen |= free & (gen >> 28);
    return gen | (gen >> 7) & RIGHT_MOVE_MASK[1];
}

//Left moving
pub fn fill_left(mut gen: u64, mut free: u64) -> u64 {
    free &= LEFT_MOVE_MASK[1];
    gen |= free & (gen >> 1);
    free &=       free >> 1;
    gen |= free & (gen >> 2);
    free &=       free >> 2;
    gen |= free & (gen >> 4);

    return gen;
}
pub fn fill_left_cap(mut gen: u64, mut free: u64) -> u64 {
    free &= LEFT_MOVE_MASK[1];
    gen |= free & (gen >> 1);
    free &=       free >> 1;
    gen |= free & (gen >> 2);
    free &=       free >> 2;
    gen |= free & (gen >> 4);

    return gen & free | (gen >> 1) & LEFT_MOVE_MASK[1];
}

pub fn fill_up_left(mut gen: u64, mut free: u64) -> u64 {
    free &= LEFT_MOVE_MASK[1];
    gen |= free & (gen <<  7);
    free &=       free <<  7;
    gen |= free & (gen << 14);
    free &=       free << 14;
    gen |= free & (gen << 28);

    return gen;
}
pub fn fill_up_left_cap(mut gen: u64, mut free: u64) -> u64 {
    free &= LEFT_MOVE_MASK[1];
    gen |= free & (gen <<  7);
    free &=       free <<  7;
    gen |= free & (gen << 14);
    free &=       free << 14;
    gen |= free & (gen << 28);

    return gen & free | (gen << 7) & LEFT_MOVE_MASK[1];
}

pub fn fill_down_left(mut gen: u64, mut free: u64) -> u64 {
    free &= LEFT_MOVE_MASK[1];
    gen |= free & (gen >>  9);
    free &=       free >>  9;
    gen |= free & (gen >> 18);
    free &=       free >> 18;
    gen |= free & (gen >> 36);

    return gen;
}
pub fn fill_down_left_cap(mut gen: u64, mut free: u64) -> u64 {
    free &= LEFT_MOVE_MASK[1];
    gen |= free & (gen >>  9);
    free &=       free >>  9;
    gen |= free & (gen >> 18);
    free &=       free >> 18;
    gen |= free & (gen >> 36);

    return gen & free | (gen >> 9) & LEFT_MOVE_MASK[1];
}