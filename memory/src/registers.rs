use logic_gates::{Arr16, multibit_basic_gates::mux16};

use crate::bit::Bit;

#[derive(Debug)]
pub struct R16 {
    bits: [Bit; 16],
}

impl R16 {
    pub fn new() -> Self {
        Self {
            bits: [
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
            ],
        }
    }

    fn to_arr(&self) -> Arr16 {
        [
            self.bits[0].prove(false, false),
            self.bits[1].prove(false, false),
            self.bits[2].prove(false, false),
            self.bits[3].prove(false, false),
            self.bits[4].prove(false, false),
            self.bits[5].prove(false, false),
            self.bits[6].prove(false, false),
            self.bits[7].prove(false, false),
            self.bits[8].prove(false, false),
            self.bits[9].prove(false, false),
            self.bits[10].prove(false, false),
            self.bits[11].prove(false, false),
            self.bits[12].prove(false, false),
            self.bits[13].prove(false, false),
            self.bits[14].prove(false, false),
            self.bits[15].prove(false, false),
        ]
    }

    fn prove_bits(&self, arr: Arr16) -> Arr16 {
        [
            self.bits[0].prove(arr[0], true),
            self.bits[1].prove(arr[1], true),
            self.bits[2].prove(arr[2], true),
            self.bits[3].prove(arr[3], true),
            self.bits[4].prove(arr[4], true),
            self.bits[5].prove(arr[5], true),
            self.bits[6].prove(arr[6], true),
            self.bits[7].prove(arr[7], true),
            self.bits[8].prove(arr[8], true),
            self.bits[9].prove(arr[9], true),
            self.bits[10].prove(arr[10], true),
            self.bits[11].prove(arr[11], true),
            self.bits[12].prove(arr[12], true),
            self.bits[13].prove(arr[13], true),
            self.bits[14].prove(arr[14], true),
            self.bits[15].prove(arr[15], true),
        ]
    }

    pub fn prove(&self, input: Arr16, load: bool) -> Arr16 {
        let arr = self.to_arr();
        let m = mux16(arr, input, load);
        self.prove_bits(m)
    }
}

#[cfg(test)]
mod tests {
    use alu::alu::{ARR16_0, ARR16_1, ARR16_MAX};

    use super::*;

    #[test]
    fn r16() {
        let r16 = R16::new();
        assert_eq!(r16.to_arr(), ARR16_0);
        assert_eq!(r16.prove(ARR16_1, true), ARR16_0);
        assert_eq!(r16.prove(ARR16_MAX, false), ARR16_1);
        assert_eq!(r16.prove(ARR16_0, false), ARR16_1);
        assert_eq!(r16.prove(ARR16_MAX, true), ARR16_1);
        assert_eq!(r16.prove(ARR16_0, false), ARR16_MAX);
    }
}
