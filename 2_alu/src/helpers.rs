use logic_gates::Arr16;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::adders::incrementer16;
use crate::alu::ARR16_0;

pub fn arr2bytes(a: Arr16) -> [u8; 2] {
    let mut arr_bytes = [0u8; 2];
    for (idx, bit) in a.iter().enumerate() {
        let byte_idx = idx / 8;
        let bit_idx = 7 - idx % 8;
        arr_bytes[byte_idx] |= (*bit as u8) << bit_idx;
    }
    arr_bytes
}

pub fn arr2int(a: Arr16) -> i16 {
    i16::from_be_bytes(arr2bytes(a))
}

pub fn int2arr(a: i16) -> Arr16 {
    let mut arr = ARR16_0;
    for (idx, byte) in a.to_be_bytes().iter().enumerate() {
        for i in 0..8 {
            let bit_idx = (idx * 8) + (7 - i);
            arr[bit_idx] = (*byte & (1u8 << i)) > 0;
        }
    }
    arr
}

pub fn arr_flip_sign(mut a: Arr16) -> Arr16 {
    for bit in a.iter_mut() {
        *bit = !*bit;
    }
    incrementer16(a)
}

pub fn rand_arr() -> Arr16 {
    let mut arr = [false; 16];
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .to_string();
    for (idx, c) in time.chars().rev().take(16).enumerate() {
        arr[idx] = c.to_digit(10).unwrap() % 2 == 0;
    }
    std::thread::sleep(Duration::from_millis(10));
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adders::incrementer16,
        alu::{ARR16_1, ARR16_M1, ARR16_MIN},
    };

    #[test]
    pub fn test_array2int() {
        let mut input = ARR16_MIN;
        for i in -(2i32.pow(15))..2i32.pow(15) {
            assert_eq!(arr2int(input), i as i16);
            input = incrementer16(input);
        }
    }

    #[test]
    pub fn test_int2arr() {
        let mut expected_out = ARR16_MIN;
        for i in -(2i32.pow(15))..2i32.pow(15) {
            assert_eq!(int2arr(i as i16), expected_out);
            expected_out = incrementer16(expected_out);
        }
    }

    #[test]
    pub fn test_flip_arr_sign() {
        let one = ARR16_1;
        let m_one = ARR16_M1;
        assert_eq!(m_one, arr_flip_sign(one));
        assert_eq!(one, arr_flip_sign(m_one));

        let iter = -(2i32.pow(15))..2i32.pow(15);
        for (i, j) in (iter.clone().map(|x| x + 1)).zip(iter.rev()) {
            assert_eq!(arr_flip_sign(int2arr(i as i16)), int2arr(j as i16));
        }
    }
}
