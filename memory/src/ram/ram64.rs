use core::fmt;
use std::str::FromStr;

use logic_gates::Arr16;
use logic_gates::multiway_basic_gates as gates;

use crate::ram::ram8::RAM8;

struct RAM64 {
    registers: [RAM8; 8],
}

impl fmt::Display for RAM64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from_str("RAM64:\n").unwrap();
        for (idx, r) in self.registers.iter().enumerate() {
            s.push_str(format!("{} {}\n", idx, r).as_str());
        }
        write!(f, "{}", s)
    }
}

impl fmt::Debug for RAM64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl RAM64 {
    pub fn new(registers: [RAM8; 8]) -> Self {
        Self { registers }
    }

    pub fn with_all(arr: Arr16) -> Self {
        let registers = [
            RAM8::with_all(arr),
            RAM8::with_all(arr),
            RAM8::with_all(arr),
            RAM8::with_all(arr),
            RAM8::with_all(arr),
            RAM8::with_all(arr),
            RAM8::with_all(arr),
            RAM8::with_all(arr),
        ];
        Self::new(registers)
    }
    pub fn prove(&self, input: Arr16, load: bool, address: [bool; 6]) -> Arr16 {
        let ram_idx = [address[0], address[1], address[2]];
        let ram_arr = gates::demux8way(load, ram_idx);
        let reg_idx = [address[3], address[4], address[5]];
        let r = [
            self.registers[0].prove(input, ram_arr[7], reg_idx),
            self.registers[1].prove(input, ram_arr[6], reg_idx),
            self.registers[2].prove(input, ram_arr[5], reg_idx),
            self.registers[3].prove(input, ram_arr[4], reg_idx),
            self.registers[4].prove(input, ram_arr[3], reg_idx),
            self.registers[5].prove(input, ram_arr[2], reg_idx),
            self.registers[6].prove(input, ram_arr[1], reg_idx),
            self.registers[7].prove(input, ram_arr[0], reg_idx),
        ];
        gates::mux8way16(r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], ram_idx)
    }
}

#[cfg(test)]
mod test {
    use alu::alu::{ARR16_0, ARR16_1, ARR16_MAX, ARR16_MIN};

    use super::*;

    #[test]
    fn ram64() {
        let ram64 = RAM64::with_all(ARR16_0);

        // load 1 in register 0, gets old val ARR16_0
        let addr_0 = [false, false, false, false, false, false];
        let old0 = ram64.prove(ARR16_1, true, addr_0);
        // should get ARR16_1
        let new0 = ram64.prove(ARR16_MAX, false, addr_0);
        assert_eq!(old0, ARR16_0);
        assert_eq!(new0, ARR16_1);

        // proving for a value multiple times doesn't change it
        let addr_2 = [false, false, false, false, true, false];
        ram64.prove(ARR16_MAX, true, addr_2);
        let new2 = ram64.prove(ARR16_0, false, addr_2);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram64.prove(ARR16_0, false, addr_2);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram64.prove(ARR16_0, false, addr_2);
        assert_eq!(new2, ARR16_MAX);

        // can access different RAM8s
        let addr_1_2 = [false, false, true, false, true, false];
        ram64.prove(ARR16_MAX, true, addr_1_2);
        let new2 = ram64.prove(ARR16_0, false, addr_1_2);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram64.prove(ARR16_0, false, addr_1_2);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram64.prove(ARR16_0, false, addr_1_2);
        assert_eq!(new2, ARR16_MAX);

        let addr_7_7 = [true, true, true, true, true, true];
        ram64.prove(ARR16_MIN, true, addr_7_7);
        let new2 = ram64.prove(ARR16_0, false, addr_7_7);
        assert_eq!(new2, ARR16_MIN);
        let new2 = ram64.prove(ARR16_0, false, addr_7_7);
        assert_eq!(new2, ARR16_MIN);
        let new2 = ram64.prove(ARR16_0, false, addr_7_7);
        assert_eq!(new2, ARR16_MIN);

        dbg!(&ram64);
        assert!(false);
    }
}
