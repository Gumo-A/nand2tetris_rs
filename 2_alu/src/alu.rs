use crate::adders;
use logic_gates::Arr16;
use logic_gates::basic_gates as bg;
use logic_gates::multibit_basic_gates as mbg;
use logic_gates::multiway_basic_gates as mwg;

pub const ARR16_0: Arr16 = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false,
];
pub const ARR16_M1: Arr16 = [
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
];
pub const ARR16_1: Arr16 = [
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
    use crate::helpers::rand_arr;

    // With these tests we will simply validate the
    // truth table on figure 2.5b of the book with 'random' inputs.

    #[test]
    pub fn test_alu_zero() {
        for _ in 0..8 {
            let bits = [true, false, true, false, true, false];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, ARR16_0);
        }
    }

    #[test]
    pub fn test_alu_one() {
        for _ in 0..8 {
            let bits = [true, true, true, true, true, true];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, ARR16_1);
        }
    }

    #[test]
    pub fn test_alu_minus_one() {
        for _ in 0..8 {
            let bits = [true, true, true, false, true, false];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, ARR16_M1);
        }
    }

    #[test]
    pub fn test_alu_x() {
        for _ in 0..8 {
            let bits = [false, false, true, true, false, false];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, x);
        }
    }

    #[test]
    pub fn test_alu_y() {
        for _ in 0..8 {
            let bits = [true, true, false, false, false, false];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, y);
        }
    }

    #[test]
    pub fn test_alu_not_x() {
        for _ in 0..8 {
            let bits = [false, false, true, true, false, true];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, mbg::not16(x));
        }
    }

    #[test]
    pub fn test_alu_not_y() {
        for _ in 0..8 {
            let bits = [true, true, false, false, false, true];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, mbg::not16(y));
        }
    }

    #[test]
    pub fn test_alu_minus_x() {
        for _ in 0..8 {
            let bits = [false, false, true, true, true, true];
            let x = rand_arr();
            let y = rand_arr();

            todo!("I have to study how to calculate the negative of a number in two's complement");
            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, x);
        }
    }

    #[test]
    pub fn test_alu_minus_y() {
        for _ in 0..8 {
            let bits = [true, true, false, false, true, true];
            let x = rand_arr();
            let y = rand_arr();

            todo!("I have to study how to calculate the negative of a number in two's complement");
            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, x);
        }
    }

    #[test]
    pub fn test_alu_x_plus_one() {
        for _ in 0..8 {
            let bits = [false, true, true, true, true, true];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, adders::incrementer16(x));
        }
    }

    #[test]
    pub fn test_alu_y_plus_one() {
        for _ in 0..8 {
            let bits = [true, true, false, true, true, true];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, adders::incrementer16(y));
        }
    }

    #[test]
    pub fn test_alu_x_minus_one() {
        for _ in 0..8 {
            let bits = [false, false, true, true, true, false];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            todo!();
            assert_eq!(result, ARR16_1);
        }
    }

    #[test]
    pub fn test_alu_y_minus_one() {
        for _ in 0..8 {
            let bits = [true, true, false, false, true, false];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            todo!();
            assert_eq!(result, ARR16_M1);
        }
    }

    #[test]
    pub fn test_alu_x_plus_y() {
        for _ in 0..8 {
            let bits = [false, false, false, false, true, false];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, adders::adder16(x, y));
        }
    }

    #[test]
    pub fn test_alu_x_minus_y() {
        for _ in 0..8 {
            let bits = [false, true, false, false, true, true];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            todo!();
            assert_eq!(result, x);
        }
    }

    #[test]
    pub fn test_alu_y_minus_x() {
        for _ in 0..8 {
            let bits = [false, false, false, true, true, true];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            todo!();
            assert_eq!(result, x);
        }
    }

    #[test]
    pub fn test_alu_x_and_y() {
        for _ in 0..8 {
            let bits = [false, false, false, false, false, false];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, mbg::and16(x, y));
        }
    }

    #[test]
    pub fn test_alu_x_or_y() {
        for _ in 0..8 {
            let bits = [false, true, false, true, false, true];
            let x = rand_arr();
            let y = rand_arr();

            let (result, _, _) = alu(x, y, bits[0], bits[1], bits[2], bits[3], bits[4], bits[5]);

            assert_eq!(result, mbg::or16(x, y));
        }
    }
}
