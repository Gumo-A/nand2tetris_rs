use core::fmt;
use std::str::FromStr;

use logic_gates::Arr16;
use logic_gates::multiway_basic_gates as gates;

use crate::ram::RamChip;
use crate::ram::ram512::RAM512;

pub struct RAM4K {
    registers: Vec<RAM512>,
}

impl fmt::Display for RAM4K {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from_str("RAM4K:\n").unwrap();
        for (idx, r) in self.registers.iter().enumerate() {
            s.push_str(format!("{} {}\n", idx, r).as_str());
        }
        write!(f, "{}", s)
    }
}

impl fmt::Debug for RAM4K {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl RamChip for RAM4K {
    fn new_with(arr: Arr16) -> Self {
        let registers = vec![
            RAM512::new_with(arr),
            RAM512::new_with(arr),
            RAM512::new_with(arr),
            RAM512::new_with(arr),
            RAM512::new_with(arr),
            RAM512::new_with(arr),
            RAM512::new_with(arr),
            RAM512::new_with(arr),
        ];
        assert_eq!(registers.len(), 8);
        Self { registers }
    }

    fn prove(&mut self, input: Arr16, load: bool, address: &[bool]) -> Arr16 {
        let address: [bool; 12] = address.try_into().unwrap();
        let ram_idx = [address[0], address[1], address[2]];
        let reg_idx = [
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
        ];
        let ram_arr = gates::demux8way(load, ram_idx);
        let r = [
            self.registers[0].prove(input, ram_arr[7], &reg_idx),
            self.registers[1].prove(input, ram_arr[6], &reg_idx),
            self.registers[2].prove(input, ram_arr[5], &reg_idx),
            self.registers[3].prove(input, ram_arr[4], &reg_idx),
            self.registers[4].prove(input, ram_arr[3], &reg_idx),
            self.registers[5].prove(input, ram_arr[2], &reg_idx),
            self.registers[6].prove(input, ram_arr[1], &reg_idx),
            self.registers[7].prove(input, ram_arr[0], &reg_idx),
        ];
        gates::mux8way16(r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], ram_idx)
    }
}

#[cfg(test)]
mod test {
    use alu::alu::{ARR16_0, ARR16_1, ARR16_MAX, ARR16_MIN};

    use super::*;

    #[test]
    fn ram4k() {
        let mut ram4k = RAM4K::new_with(ARR16_0);

        // load 1 in register 0, gets old val ARR16_0
        let addr_0000 = [
            false, false, false, false, false, false, false, false, false, false, false, false,
        ];
        let old0 = ram4k.prove(ARR16_1, true, &addr_0000);
        // should get ARR16_1
        let new0 = ram4k.prove(ARR16_MAX, false, &addr_0000);
        assert_eq!(old0, ARR16_0);
        assert_eq!(new0, ARR16_1);

        // proving for a value multiple times doesn't change it
        let addr_0200 = [
            false, false, false, false, true, false, false, false, false, false, false, false,
        ];
        ram4k.prove(ARR16_MAX, true, &addr_0200);
        let new2 = ram4k.prove(ARR16_0, false, &addr_0200);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram4k.prove(ARR16_0, false, &addr_0200);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram4k.prove(ARR16_0, false, &addr_0200);
        assert_eq!(new2, ARR16_MAX);

        // can access different RAM64s
        let addr_1200 = [
            false, false, true, false, true, false, false, false, false, false, false, false,
        ];
        ram4k.prove(ARR16_MAX, true, &addr_1200);
        let new2 = ram4k.prove(ARR16_0, false, &addr_1200);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram4k.prove(ARR16_0, false, &addr_1200);
        assert_eq!(new2, ARR16_MAX);
        let new2 = ram4k.prove(ARR16_0, false, &addr_1200);
        assert_eq!(new2, ARR16_MAX);

        let addr_7777 = [
            true, true, true, true, true, true, true, true, true, true, true, true,
        ];
        ram4k.prove(ARR16_MIN, true, &addr_7777);
        let new2 = ram4k.prove(ARR16_0, false, &addr_7777);
        assert_eq!(new2, ARR16_MIN);
        let new2 = ram4k.prove(ARR16_0, false, &addr_7777);
        assert_eq!(new2, ARR16_MIN);
        let new2 = ram4k.prove(ARR16_0, false, &addr_7777);
        assert_eq!(new2, ARR16_MIN);
    }
}
