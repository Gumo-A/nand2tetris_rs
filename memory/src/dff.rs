#[derive(Debug)]
pub struct DFF {
    pub out: bool,
}

impl DFF {
    pub fn new(state: bool) -> Self {
        Self { out: state }
    }

    pub fn prove(&mut self, input: bool) -> bool {
        let out = self.out;
        self.out = input;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dff() {
        let mut dff = DFF::new(false);
        assert_eq!(dff.out, false);
        assert_eq!(dff.prove(true), false);
        assert_eq!(dff.out, true);

        let mut dff = DFF::new(true);
        assert_eq!(dff.out, true);
        assert_eq!(dff.prove(false), true);
        assert_eq!(dff.out, false);

        let mut dff = DFF::new(true);
        assert_eq!(dff.out, true);
        assert_eq!(dff.prove(true), true);
        assert_eq!(dff.out, true);

        let mut dff = DFF::new(false);
        assert_eq!(dff.out, false);
        assert_eq!(dff.prove(false), false);
        assert_eq!(dff.out, false);
    }
}
