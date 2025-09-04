use crate::Arr16;
use crate::basic_gates::*;

pub fn not16(a: Arr16) -> Arr16 {
    [
        not(a[0]),
        not(a[1]),
        not(a[2]),
        not(a[3]),
        not(a[4]),
        not(a[5]),
        not(a[6]),
        not(a[7]),
        not(a[8]),
        not(a[9]),
        not(a[10]),
        not(a[11]),
        not(a[12]),
        not(a[13]),
        not(a[14]),
        not(a[15]),
    ]
}

pub fn and16(a: Arr16, b: Arr16) -> Arr16 {
    [
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ]
}

pub fn or16(a: Arr16, b: Arr16) -> Arr16 {
    [
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ]
}

pub fn xor16(a: Arr16, b: Arr16) -> Arr16 {
    [
        xor(a[0], b[0]),
        xor(a[1], b[1]),
        xor(a[2], b[2]),
        xor(a[3], b[3]),
        xor(a[4], b[4]),
        xor(a[5], b[5]),
        xor(a[6], b[6]),
        xor(a[7], b[7]),
        xor(a[8], b[8]),
        xor(a[9], b[9]),
        xor(a[10], b[10]),
        xor(a[11], b[11]),
        xor(a[12], b[12]),
        xor(a[13], b[13]),
        xor(a[14], b[14]),
        xor(a[15], b[15]),
    ]
}

pub fn mux16(a: Arr16, b: Arr16, s: bool) -> Arr16 {
    [
        mux(a[0], b[0], s),
        mux(a[1], b[1], s),
        mux(a[2], b[2], s),
        mux(a[3], b[3], s),
        mux(a[4], b[4], s),
        mux(a[5], b[5], s),
        mux(a[6], b[6], s),
        mux(a[7], b[7], s),
        mux(a[8], b[8], s),
        mux(a[9], b[9], s),
        mux(a[10], b[10], s),
        mux(a[11], b[11], s),
        mux(a[12], b[12], s),
        mux(a[13], b[13], s),
        mux(a[14], b[14], s),
        mux(a[15], b[15], s),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not16() {
        let input = [true; 16];
        let output = not16(input);
        for i in 0..output.len() {
            assert_eq!(output[i], false);
        }
        let input = [false; 16];
        let output = not16(input);
        for i in 0..output.len() {
            assert_eq!(output[i], true);
        }
    }

    #[test]
    fn test_and16() {
        let a = [true; 16];
        let b = [true; 16];
        let output = and16(a, b);
        for i in 0..output.len() {
            assert_eq!(output[i], true);
        }
        let a = [true; 16];
        let b = [false; 16];
        let output = and16(a, b);
        for i in 0..output.len() {
            assert_eq!(output[i], false);
        }
        let a = [true; 16];
        let b = [
            true, false, false, false, false, true, false, false, false, false, false, false,
            false, false, false, true,
        ];
        let output = and16(a, b);
        assert_eq!(output[0], true);
        assert_eq!(output[1], false);
        assert_eq!(output[2], false);
        assert_eq!(output[3], false);
        assert_eq!(output[4], false);
        assert_eq!(output[5], true);
        assert_eq!(output[6], false);
        assert_eq!(output[7], false);
        assert_eq!(output[8], false);
        assert_eq!(output[9], false);
        assert_eq!(output[10], false);
        assert_eq!(output[11], false);
        assert_eq!(output[12], false);
        assert_eq!(output[13], false);
        assert_eq!(output[14], false);
        assert_eq!(output[15], true);
    }

    #[test]
    fn test_or16() {
        let a = [true; 16];
        let b = [true; 16];
        let output = or16(a, b);
        for i in 0..output.len() {
            assert_eq!(output[i], true);
        }
        let a = [true; 16];
        let b = [false; 16];
        let output = or16(a, b);
        for i in 0..output.len() {
            assert_eq!(output[i], true);
        }
        let a = [false; 16];
        let b = [
            true, false, false, false, false, true, false, false, false, false, false, false,
            false, false, false, true,
        ];
        let output = or16(a, b);
        assert_eq!(output[0], true);
        assert_eq!(output[1], false);
        assert_eq!(output[2], false);
        assert_eq!(output[3], false);
        assert_eq!(output[4], false);
        assert_eq!(output[5], true);
        assert_eq!(output[6], false);
        assert_eq!(output[7], false);
        assert_eq!(output[8], false);
        assert_eq!(output[9], false);
        assert_eq!(output[10], false);
        assert_eq!(output[11], false);
        assert_eq!(output[12], false);
        assert_eq!(output[13], false);
        assert_eq!(output[14], false);
        assert_eq!(output[15], true);
    }

    #[test]
    fn test_xor16() {
        let a = [true; 16];
        let b = [true; 16];
        let output = xor16(a, b);
        for i in 0..output.len() {
            assert_eq!(output[i], false);
        }
        let a = [
            true, true, false, true, true, true, true, true, true, true, true, true, true, true,
            true, false,
        ];
        let b = [
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false, true,
        ];
        let output = xor16(a, b);
        for i in 0..output.len() {
            assert_eq!(output[i], true);
        }
    }

    #[test]
    fn test_mux16() {
        let a = [
            true, true, false, true, true, true, true, false, false, true, false, true, true, true,
            true, false,
        ];
        let b = [
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false, true,
        ];
        let output = mux16(a, b, false);
        for i in 0..output.len() {
            assert_eq!(output[i], a[i]);
        }

        let output = mux16(a, b, true);
        for i in 0..output.len() {
            assert_eq!(output[i], b[i]);
        }
    }
}
