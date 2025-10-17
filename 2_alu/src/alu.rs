use crate::adders;
use logic_gates::Arr16;
use logic_gates::basic_gates as bg;
use logic_gates::multibit_basic_gates as mbg;
use logic_gates::multiway_basic_gates as mwg;

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const ARR16_M1: Arr16 = [
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
];
const ARR16_0: Arr16 = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false,
];
const ARR16_1: Arr16 = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true,
];

pub fn alu(
    x: Arr16,
    y: Arr16,
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> (Arr16, bool, bool) {
    let x = mbg::mux16(x, ARR16_0, zx);
    let x = mbg::mux16(x, mbg::not16(x), nx);

    let y = mbg::mux16(y, ARR16_0, zy);
    let y = mbg::mux16(y, mbg::not16(y), ny);

    let result = mbg::mux16(mbg::and16(x, y), adders::adder16(x, y), f);
    let result = mbg::mux16(result, mbg::not16(result), no);

    let zr = bg::not(mwg::or16way(result));
    let ng = result[0];

    (result, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_int() -> Arr16 {
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

    // With these tests we will simply validate the
    // truth table on figure 2.5b of the book.

    #[test]
    pub fn test_alu_zero() {
        let bits = [true, false, true, false, true, false];
        let x = rand_int();
        let y = rand_int();

        let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

        assert_eq!(result, ARR16_0);
    }

    #[test]
    pub fn test_alu_one() {
        let bits = [true, true, true, true, true, true];
        let x = rand_int();
        let y = rand_int();

        let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

        assert_eq!(result, ARR16_1);
    }

    #[test]
    pub fn test_alu_minus_one() {
        let bits = [true, true, true, false, true, false];
        let x = rand_int();
        let y = rand_int();

        let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

        assert_eq!(result, ARR16_M1);
    }
}
