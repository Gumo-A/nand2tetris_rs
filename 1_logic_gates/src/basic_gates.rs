/// The foundation of our system.
/// Everything here is built from a simple NAND gate,
/// which will be the only part of the system that
/// won't be contructed from our "chips"
use crate::nand::nand;

/// NOT truth table
/// 0 1
/// 1 0
pub fn not(a: bool) -> bool {
    nand(a, a)
}

/// AND truth table
/// 00 0
/// 01 0
/// 10 0
/// 11 1
pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

/// OR truth table
/// 00 0
/// 01 1
/// 10 1
/// 11 1
pub fn or(a: bool, b: bool) -> bool {
    not(and(not(a), not(b)))
}

/// XOR truth table
/// 00 0
/// 01 1
/// 10 1
/// 11 0
pub fn xor(a: bool, b: bool) -> bool {
    not(and(or(not(a), b), or(a, not(b))))
}

/// MULTIPLEXER truth table
/// a b s | mux
/// -------
/// 0 0 0 |  0
/// 0 1 0 |  0
/// 1 0 0 |  1
/// 1 1 0 |  1
/// 0 0 1 |  0
/// 0 1 1 |  1
/// 1 0 1 |  0
/// 1 1 1 |  1
pub fn mux(a: bool, b: bool, s: bool) -> bool {
    not(xor(or(s, a), or(not(s), b)))
}

/// i s |
/// 0 0 | 00
/// 0 1 | 00
/// 1 0 | 01
/// 1 1 | 10
///
/// s
/// 0 | 01
/// 1 | 10
/// if i == 0, out = 00
pub fn demux(i: bool, s: bool) -> [bool; 2] {
    [not(xor(s, or(not(s), i))), not(xor(not(s), or(s, i)))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(not(false), true);
        assert_eq!(not(true), false);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(false, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(true, false), false);
        assert_eq!(and(true, true), true);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(true, true), true);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(true, true), false);
    }

    #[test]
    fn test_multiplexer() {
        assert_eq!(mux(false, false, false), false);
        assert_eq!(mux(false, true, false), false);
        assert_eq!(mux(true, false, false), true);
        assert_eq!(mux(true, true, false), true);
        assert_eq!(mux(false, false, true), false);
        assert_eq!(mux(false, true, true), true);
        assert_eq!(mux(true, false, true), false);
        assert_eq!(mux(true, true, true), true);
    }

    #[test]
    fn test_demultiplexer() {
        assert_eq!(demux(false, false), [false, false]);
        assert_eq!(demux(false, true), [false, false]);
        assert_eq!(demux(true, false), [false, true]);
        assert_eq!(demux(true, true), [true, false]);
    }
}
