use logic_gates::Arr16;

pub mod ram16k;
pub mod ram4k;
pub mod ram512;
pub mod ram64;
pub mod ram8;

pub trait RamChip {
    fn new_with(arr: Arr16) -> Self;
    fn prove(&mut self, input: Arr16, load: bool, address: &[bool]) -> Arr16;
}
