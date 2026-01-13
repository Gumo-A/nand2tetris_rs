use core::fmt;
use std::str::FromStr;

use logic_gates::Arr16;
use logic_gates::multiway_basic_gates as gates;

use crate::ram::RamChip;
use crate::registers::R16;

pub struct RAM8 {
    registers: Vec<R16>,
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

impl RamChip for RAM8 {
    fn new_with(arr: Arr16) -> Self {
        let registers = vec![
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
            R16::with_arr(arr),
        ];
        assert_eq!(registers.len(), 8);
        Self { registers }
    }

    fn prove(&mut self, input: Arr16, load: bool, address: &[bool]) -> Arr16 {
        let address = address.try_into().unwrap();
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
        let mut ram8 = RAM8::new_with(ARR16_0);

        // load 1 in register 0, gets old val ARR16_0
        let addr_0 = [false, false, false];
        let old0 = ram8.prove(ARR16_1, true, &addr_0);
        // should get ARR16_1
        let new0 = ram8.prove(ARR16_MAX, false, &addr_0);
        assert_eq!(old0, ARR16_0);
        assert_eq!(new0, ARR16_1);

        // proving for a value multiple times doesn't change it
        let addr_2 = [false, true, false];
        ram8.prove(ARR16_MAX, true, &addr_2);
        let new2 = ram8.prove(ARR16_0, false, &addr_2);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram8.prove(ARR16_0, false, &addr_2);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram8.prove(ARR16_0, false, &addr_2);
        assert_eq!(new2, ARR16_MAX);
    }
}
