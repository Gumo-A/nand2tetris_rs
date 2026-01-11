use logic_gates::basic_gates::mux;

use crate::dff::DFF;

#[derive(Debug)]
pub struct Bit {
    dff: DFF,
}

impl Bit {
    pub fn new(state: bool) -> Self {
        Self {
            dff: DFF::new(state),
        }
    }

    pub fn prove(&self, input: bool, load: bool) -> bool {
        let m = mux(*self.dff.out.borrow(), input, load);
        self.dff.prove(m)
    }

    pub fn str(&self) -> char {
        if self.prove(false, false) { '1' } else { '0' }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit() {
        let bit = Bit::new(true);
        assert_eq!(bit.prove(false, true), true);
        assert_eq!(bit.prove(false, false), false);
        assert_eq!(bit.prove(true, true), false);
        assert_eq!(bit.prove(true, true), true);
    }
}
