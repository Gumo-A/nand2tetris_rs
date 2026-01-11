use core::fmt;
use std::str::FromStr;

use logic_gates::Arr16;
use logic_gates::multiway_basic_gates as gates;

use crate::registers::R16;

pub struct RAM8 {
    registers: [R16; 8],
}

impl fmt::Display for RAM8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from_str("RAM8:\n").unwrap();
        for (idx, r) in self.registers.iter().enumerate() {
            s.push_str(format!("{} {}\n", idx, r).as_str());
        }
        write!(f, "{}", s)
    }
}

impl fmt::Debug for RAM8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl RAM8 {
    pub fn new(registers: [R16; 8]) -> Self {
        Self { registers }
    }

    pub fn with_all(arr: Arr16) -> Self {
        let registers = [
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
        ];
        Self::new(registers)
    }

    pub fn prove(&self, input: Arr16, load: bool, address: [bool; 3]) -> Arr16 {
        let load_arr = gates::demux8way(load, address);
        let r = [
            self.registers[0].prove(input, load_arr[7]),
            self.registers[1].prove(input, load_arr[6]),
            self.registers[2].prove(input, load_arr[5]),
            self.registers[3].prove(input, load_arr[4]),
            self.registers[4].prove(input, load_arr[3]),
            self.registers[5].prove(input, load_arr[2]),
            self.registers[6].prove(input, load_arr[1]),
            self.registers[7].prove(input, load_arr[0]),
        ];
        gates::mux8way16(r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], address)
    }
}

#[cfg(test)]
mod test {
    use alu::alu::{ARR16_0, ARR16_1, ARR16_MAX};

    use super::*;

    #[test]
    fn ram8() {
        let ram8 = RAM8::with_all(ARR16_0);

        // load 1 in register 0, gets old val ARR16_0
        let old0 = ram8.prove(ARR16_1, true, [false, false, false]);
        // should get ARR16_1
        let new0 = ram8.prove(ARR16_MAX, false, [false, false, false]);
        assert_eq!(old0, ARR16_0);
        assert_eq!(new0, ARR16_1);

        // proving for a value multiple times doesn't change it
        ram8.prove(ARR16_MAX, true, [false, true, false]);
        let new2 = ram8.prove(ARR16_0, false, [false, true, false]);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram8.prove(ARR16_0, false, [false, true, false]);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram8.prove(ARR16_0, false, [false, true, false]);
        assert_eq!(new2, ARR16_MAX);
    }
}
