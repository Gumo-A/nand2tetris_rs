use crate::basic_gates::*;
use crate::multibit_basic_gates::*;
use crate::{Arr4, Arr8, Arr16};

pub fn or8way(i: Arr8) -> bool {
    or(
        or(or(i[0], i[1]), or(i[2], i[3])),
        or(or(i[4], i[5]), or(i[6], i[7])),
    )
}

pub fn and8way(i: Arr8) -> bool {
    and(
        and(and(i[0], i[1]), and(i[2], i[3])),
        and(and(i[4], i[5]), and(i[6], i[7])),
    )
}

/// s.0 s.1 |   |
/// ------------|
///  0   0  | a |
///  0   1  | b |
///  1   0  | c |
///  1   1  | d |
pub fn mux4way16(a: Arr16, b: Arr16, c: Arr16, d: Arr16, s: [bool; 2]) -> Arr16 {
    mux16(mux16(a, b, s[1]), mux16(c, d, s[1]), s[0])
}

/// s.0 s.1 s.2 |
///  0   0   0  | a
///  0   0   1  | b
///  0   1   0  | c
///  0   1   1  | d
///  1   0   0  | e
///  1   0   1  | f
///  1   1   0  | g
///  1   1   1  | h
pub fn mux8way16(
    a: Arr16,
    b: Arr16,
    c: Arr16,
    d: Arr16,
    e: Arr16,
    f: Arr16,
    g: Arr16,
    h: Arr16,
    s: [bool; 3],
) -> Arr16 {
    mux16(
        mux4way16(a, b, c, d, [s[1], s[2]]),
        mux4way16(e, f, g, h, [s[1], s[2]]),
        s[0],
    )
}

/// s.0 s.1
///  0   0  | 000i
///  0   1  | 00i0
///  1   0  | 0i00
///  1   1  | i000
pub fn demux4way(i: bool, s: [bool; 2]) -> Arr4 {
    let not_s0 = not(s[0]);
    let not_s1 = not(s[1]);
    [
        demux(i, or(not_s0, not_s1))[1],
        demux(i, or(not_s0, s[1]))[1],
        demux(i, or(s[0], not_s1))[1],
        demux(i, or(s[0], s[1]))[1],
    ]
}

/// s.0 s.1 s.2   
///  0   0   0  | 0000 000i
///  0   0   1  | 0000 00i0
///  0   1   0  | 0000 0i00
///  0   1   1  | 0000 i000
///  1   0   0  | 000i 0000
///  1   0   1  | 00i0 0000
///  1   1   0  | 0i00 0000
///  1   1   1  | i000 0000
pub fn demux8way(i: bool, s: [bool; 3]) -> Arr8 {
    let not_s0 = not(s[0]);
    let not_s1 = not(s[1]);
    let not_s2 = not(s[2]);
    let or_nots0s1 = or(not_s0, not_s1);
    let or_nots0_s1 = or(not_s0, s[1]);
    let or_s0_nots1 = or(s[0], not_s1);
    let or_s0s1 = or(s[0], s[1]);
    [
        demux(i, or(or_nots0s1, not_s2))[1],
        demux(i, or(or_nots0s1, s[2]))[1],
        demux(i, or(or_nots0_s1, not_s2))[1],
        demux(i, or(or_nots0_s1, s[2]))[1],
        demux(i, or(or_s0_nots1, not_s2))[1],
        demux(i, or(or_s0_nots1, s[2]))[1],
        demux(i, or(or_s0s1, not_s2))[1],
        demux(i, or(or_s0s1, s[2]))[1],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or8way() {
        let input = [true, false, false, false, false, false, false, false];
        let output = or8way(input);
        assert_eq!(output, true);

        let input = [false, false, false, false, true, false, false, false];
        let output = or8way(input);
        assert_eq!(output, true);

        let input = [false; 8];
        let output = or8way(input);
        assert_eq!(output, false);
    }
    #[test]
    fn test_and8way() {
        let input = [true, false, false, false, false, false, false, false];
        let output = and8way(input);
        assert_eq!(output, false);

        let input = [false, false, false, false, true, false, false, false];
        let output = and8way(input);
        assert_eq!(output, false);

        let input = [false; 8];
        let output = and8way(input);
        assert_eq!(output, false);

        let input = [true; 8];
        let output = and8way(input);
        assert_eq!(output, true);
    }

    #[test]
    fn test_mux4way16() {
        let a = [true; 16];
        let b = [false; 16];
        let mut c = [true; 16];
        c[0] = false;
        c[15] = false;
        let mut d = [false; 16];
        d[0] = true;
        d[15] = true;
        let output = mux4way16(a, b, c, d, [false, false]);

        for i in 0..output.len() {
            assert_eq!(output[i], a[i]);
        }

        let output = mux4way16(a, b, c, d, [false, true]);
        for i in 0..output.len() {
            assert_eq!(output[i], b[i]);
        }

        let output = mux4way16(a, b, c, d, [true, false]);
        for i in 0..output.len() {
            assert_eq!(output[i], c[i]);
        }

        let output = mux4way16(a, b, c, d, [true, true]);
        for i in 0..output.len() {
            assert_eq!(output[i], d[i]);
        }
    }

    #[test]
    fn test_mux8way16() {
        let a = [true; 16];
        let b = [false; 16];
        let mut c = [true; 16];
        c[0] = false;
        c[15] = false;
        let mut d = [false; 16];
        d[0] = true;
        d[15] = true;
        let mut e = [true; 16];
        e[1] = false;
        let mut f = [false; 16];
        f[1] = true;
        let mut g = [true; 16];
        g[0] = false;
        g[1] = false;
        g[15] = false;
        let mut h = [false; 16];
        h[0] = true;
        h[1] = true;
        h[15] = true;
        let output = mux8way16(a, b, c, d, e, f, g, h, [false, false, false]);
        for i in 0..output.len() {
            assert_eq!(output[i], a[i]);
        }

        let output = mux8way16(a, b, c, d, e, f, g, h, [false, false, true]);
        for i in 0..output.len() {
            assert_eq!(output[i], b[i]);
        }

        let output = mux8way16(a, b, c, d, e, f, g, h, [false, true, false]);
        for i in 0..output.len() {
            assert_eq!(output[i], c[i]);
        }
        let output = mux8way16(a, b, c, d, e, f, g, h, [false, true, true]);
        for i in 0..output.len() {
            assert_eq!(output[i], d[i]);
        }

        let output = mux8way16(a, b, c, d, e, f, g, h, [true, false, false]);
        for i in 0..output.len() {
            assert_eq!(output[i], e[i]);
        }

        let output = mux8way16(a, b, c, d, e, f, g, h, [true, false, true]);
        for i in 0..output.len() {
            assert_eq!(output[i], f[i]);
        }

        let output = mux8way16(a, b, c, d, e, f, g, h, [true, true, false]);
        for i in 0..output.len() {
            assert_eq!(output[i], g[i]);
        }

        let output = mux8way16(a, b, c, d, e, f, g, h, [true, true, true]);
        for i in 0..output.len() {
            assert_eq!(output[i], h[i]);
        }
    }
    #[test]
    fn test_demux4way() {
        let output = demux4way(true, [false, false]);
        assert_eq!([false, false, false, true], output);

        let output = demux4way(true, [false, true]);
        assert_eq!([false, false, true, false], output);

        let output = demux4way(true, [true, false]);
        assert_eq!([false, true, false, false], output);

        let output = demux4way(true, [true, true]);
        assert_eq!([true, false, false, false], output);

        let output = demux4way(false, [false, false]);
        assert_eq!([false, false, false, false], output);

        let output = demux4way(false, [false, true]);
        assert_eq!([false, false, false, false], output);

        let output = demux4way(false, [true, false]);
        assert_eq!([false, false, false, false], output);

        let output = demux4way(false, [true, true]);
        assert_eq!([false, false, false, false], output);
    }

    #[test]
    fn test_demux8way() {
        let output = demux8way(true, [false, false, false]);
        assert_eq!(
            [false, false, false, false, false, false, false, true],
            output
        );

        let output = demux8way(true, [false, false, true]);
        assert_eq!(
            [false, false, false, false, false, false, true, false],
            output
        );

        let output = demux8way(true, [false, true, false]);
        assert_eq!(
            [false, false, false, false, false, true, false, false],
            output
        );

        let output = demux8way(true, [false, true, true]);
        assert_eq!(
            [false, false, false, false, true, false, false, false],
            output
        );

        let output = demux8way(true, [true, false, false]);
        assert_eq!(
            [false, false, false, true, false, false, false, false],
            output
        );

        let output = demux8way(true, [true, false, true]);
        assert_eq!(
            [false, false, true, false, false, false, false, false],
            output
        );

        let output = demux8way(true, [true, true, false]);
        assert_eq!(
            [false, true, false, false, false, false, false, false],
            output
        );

        let output = demux8way(true, [true, true, true]);
        assert_eq!(
            [true, false, false, false, false, false, false, false],
            output
        );

        let output = demux8way(false, [false, false, false]);
        assert_eq!(
            [false, false, false, false, false, false, false, false],
            output
        );

        let output = demux8way(false, [false, false, true]);
        assert_eq!(
            [false, false, false, false, false, false, false, false],
            output
        );

        let output = demux8way(false, [false, true, false]);
        assert_eq!(
            [false, false, false, false, false, false, false, false],
            output
        );

        let output = demux8way(false, [false, true, true]);
        assert_eq!(
            [false, false, false, false, false, false, false, false],
            output
        );

        let output = demux8way(false, [true, false, false]);
        assert_eq!(
            [false, false, false, false, false, false, false, false],
            output
        );

        let output = demux8way(false, [true, false, true]);
        assert_eq!(
            [false, false, false, false, false, false, false, false],
            output
        );

        let output = demux8way(false, [true, true, false]);
        assert_eq!(
            [false, false, false, false, false, false, false, false],
            output
        );

        let output = demux8way(false, [true, true, true]);
        assert_eq!(
            [false, false, false, false, false, false, false, false],
            output
        );
    }
}
