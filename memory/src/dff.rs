use std::cell::RefCell;

#[derive(Debug)]
pub struct DFF {
    pub out: RefCell<bool>,
}

impl DFF {
    pub fn new(state: bool) -> Self {
        Self {
            out: RefCell::new(state),
        }
    }

    pub fn prove(&self, input: bool) -> bool {
        let out = *self.out.borrow();
        *self.out.borrow_mut() = input;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dff() {
        let dff = DFF::new(false);
        assert_eq!(*dff.out.borrow(), false);
        assert_eq!(dff.prove(true), false);
        assert_eq!(*dff.out.borrow(), true);

        let dff = DFF::new(true);
        assert_eq!(*dff.out.borrow(), true);
        assert_eq!(dff.prove(false), true);
        assert_eq!(*dff.out.borrow(), false);

        let dff = DFF::new(true);
        assert_eq!(*dff.out.borrow(), true);
        assert_eq!(dff.prove(true), true);
        assert_eq!(*dff.out.borrow(), true);

        let dff = DFF::new(false);
        assert_eq!(*dff.out.borrow(), false);
        assert_eq!(dff.prove(false), false);
        assert_eq!(*dff.out.borrow(), false);
    }
}
