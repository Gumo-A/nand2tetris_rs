use logic_gates::basic_gates as bg;
use logic_gates::{Arr4, Arr8, Arr16};

/// a b | carry sum
/// 0 0 |   0    0
/// 0 1 |   0    1
/// 1 0 |   0    1
/// 1 1 |   1    0
///
/// Carry is [0], Sum is [1]
pub fn half_adder(a: bool, b: bool) -> [bool; 2] {
    [bg::and(a, b), bg::xor(a, b)]
}

/// a b c | ab c | absc | carry sum
///
/// 0 0 0 | 00 0 |  00  |   0    0
/// 0 0 1 | 00 1 |  01  |   0    1
/// 0 1 0 | 01 0 |  01  |   0    1
/// 1 0 0 | 01 0 |  01  |   0    1
/// 0 1 1 | 01 1 |  10  |   1    0
/// 1 0 1 | 01 1 |  10  |   1    0
/// 1 1 0 | 10 0 |  00  |   1    0
/// 1 1 1 | 10 1 |  01  |   1    1
pub fn full_adder(a: bool, b: bool, c: bool) -> [bool; 2] {
    // original solution:
    // [
    //     bg::xor(half_adder(a, b)[0], half_adder(half_adder(a, b)[1], c)[0]),
    //     half_adder(half_adder(a, b)[1], c)[1],
    // ]
    // my original solution was exactly the same as the optimal one, but with xor.
    // I forgot to check the rest of the cases to verify that
    // the solution didnt require xor. I changed to or because
    // it is a bit faster.
    //
    // the book says only two half adder gates are needed,
    // and that is true in this solution, it's just that
    // I didn't want to use variable assaignment for gate
    // construction code
    //
    // I use variable asaignment now because that better
    // simulates how the gates are contructed physically.

    let a_b = half_adder(a, b);
    let absum_c = half_adder(a_b[1], c);
    [bg::or(a_b[0], absum_c[0]), absum_c[1]]
}

pub fn adder4(a: Arr4, b: Arr4) -> Arr4 {
    let _0 = full_adder(a[3], b[3], false);
    let _1 = full_adder(a[2], b[2], _0[0]);
    let _2 = full_adder(a[1], b[1], _1[0]);
    let _3 = full_adder(a[0], b[0], _2[0]);
    [_3[1], _2[1], _1[1], _0[1]]
}

pub fn adder8(a: Arr8, b: Arr8) -> Arr8 {
    let _0 = full_adder(a[7], b[7], false);
    let _1 = full_adder(a[6], b[6], _0[0]);
    let _2 = full_adder(a[5], b[5], _1[0]);
    let _3 = full_adder(a[4], b[4], _2[0]);

    let _4 = full_adder(a[3], b[3], _3[0]);
    let _5 = full_adder(a[2], b[2], _4[0]);
    let _6 = full_adder(a[1], b[1], _5[0]);
    let _7 = full_adder(a[0], b[0], _6[0]);
    [_7[1], _6[1], _5[1], _4[1], _3[1], _2[1], _1[1], _0[1]]
}

pub fn adder16(a: Arr16, b: Arr16) -> Arr16 {
    let _0 = full_adder(a[15], b[15], false);
    let _1 = full_adder(a[14], b[14], _0[0]);
    let _2 = full_adder(a[13], b[13], _1[0]);
    let _3 = full_adder(a[12], b[12], _2[0]);

    let _4 = full_adder(a[11], b[11], _3[0]);
    let _5 = full_adder(a[10], b[10], _4[0]);
    let _6 = full_adder(a[9], b[9], _5[0]);
    let _7 = full_adder(a[8], b[8], _6[0]);

    let _8 = full_adder(a[7], b[7], _7[0]);
    let _9 = full_adder(a[6], b[6], _8[0]);
    let _10 = full_adder(a[5], b[5], _9[0]);
    let _11 = full_adder(a[4], b[4], _10[0]);

    let _12 = full_adder(a[3], b[3], _11[0]);
    let _13 = full_adder(a[2], b[2], _12[0]);
    let _14 = full_adder(a[1], b[1], _13[0]);
    let _15 = full_adder(a[0], b[0], _14[0]);
    [
        _15[1], _14[1], _13[1], _12[1], _11[1], _10[1], _9[1], _8[1], _7[1], _6[1], _5[1], _4[1],
        _3[1], _2[1], _1[1], _0[1],
    ]
}

pub fn incrementer4(a: Arr4) -> Arr4 {
    adder4(a, [false, false, false, true])
}

pub fn incrementer8(a: Arr8) -> Arr8 {
    adder8(a, [false, false, false, false, false, false, false, true])
}

pub fn incrementer16(a: Arr16) -> Arr16 {
    adder16(
        a,
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_half_adder() {
        assert_eq!(half_adder(false, false), [false, false]);
        assert_eq!(half_adder(false, true), [false, true]);
        assert_eq!(half_adder(true, false), [false, true]);
        assert_eq!(half_adder(true, true), [true, false]);
    }

    #[test]
    pub fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), [false, false]);
        assert_eq!(full_adder(false, false, true), [false, true]);
        assert_eq!(full_adder(false, true, false), [false, true]);
        assert_eq!(full_adder(false, true, true), [true, false]);
        assert_eq!(full_adder(true, false, false), [false, true]);
        assert_eq!(full_adder(true, false, true), [true, false]);
        assert_eq!(full_adder(true, true, false), [true, false]);
        assert_eq!(full_adder(true, true, true), [true, true]);
    }

    #[test]
    pub fn test_adder4() {
        let _0 = [false, false, false, false];
        let _1 = [false, false, false, true];
        let _15 = [true, true, true, true];

        let _1 = adder4(_0, _1);
        let _2 = adder4(_1, _1);
        let _4 = adder4(_2, _2);
        let _8 = adder4(_4, _4);

        let r = _0;
        let r = adder4(r, _8);
        let r = adder4(r, _4);
        let r = adder4(r, _2);
        let r = adder4(r, _1);
        let r = adder4(r, _0);

        assert_eq!(r, _15);
        assert_eq!(adder4(r, _1), _0);
    }

    #[test]
    pub fn test_adder8() {
        let _0 = [false, false, false, false, false, false, false, false];
        let _1 = [false, false, false, false, false, false, false, true];
        let _255 = [true, true, true, true, true, true, true, true];

        let _0 = adder8(_0, _0);
        let _1 = adder8(_0, _1);
        let _2 = adder8(_1, _1);
        let _4 = adder8(_2, _2);
        let _8 = adder8(_4, _4);
        let _16 = adder8(_8, _8);
        let _32 = adder8(_16, _16);
        let _64 = adder8(_32, _32);
        let _128 = adder8(_64, _64);

        let r = _0;
        let r = adder8(r, _8);
        let r = adder8(r, _4);
        let r = adder8(r, _2);
        let r = adder8(r, _1);

        let r = adder8(r, _128);
        let r = adder8(r, _64);
        let r = adder8(r, _32);
        let r = adder8(r, _16);

        let r = adder8(r, _0);

        assert_eq!(r, _255);
        assert_eq!(adder8(r, _1), _0);
    }

    #[test]
    pub fn test_adder16() {
        let _0 = [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ];
        let _1 = [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true,
        ];
        let _65535 = [
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true,
        ];

        let _0 = adder16(_0, _0);
        let _1 = adder16(_0, _1);
        let _2 = adder16(_1, _1);
        let _4 = adder16(_2, _2);
        let _8 = adder16(_4, _4);
        let _16 = adder16(_8, _8);
        let _32 = adder16(_16, _16);
        let _64 = adder16(_32, _32);
        let _128 = adder16(_64, _64);
        let _256 = adder16(_128, _128);
        let _512 = adder16(_256, _256);
        let _1024 = adder16(_512, _512);
        let _2048 = adder16(_1024, _1024);
        let _4096 = adder16(_2048, _2048);
        let _8192 = adder16(_4096, _4096);
        let _16384 = adder16(_8192, _8192);
        let _32768 = adder16(_16384, _16384);

        let r = _0;
        let r = adder16(r, _128);
        let r = adder16(r, _64);
        let r = adder16(r, _32);
        let r = adder16(r, _16);
        let r = adder16(r, _0);

        let r = adder16(r, _2048);
        let r = adder16(r, _1024);
        let r = adder16(r, _512);
        let r = adder16(r, _256);

        let r = adder16(r, _32768);
        let r = adder16(r, _16384);
        let r = adder16(r, _8192);
        let r = adder16(r, _4096);

        let r = adder16(r, _8);
        let r = adder16(r, _4);
        let r = adder16(r, _2);
        let r = adder16(r, _1);

        assert_eq!(r, _65535);
        assert_eq!(adder16(r, _1), _0);
    }

    #[test]
    pub fn test_incrementer4() {
        let _0 = [false, false, false, false];
        let _15 = [true, true, true, true];
        let mut r = _0;
        for _ in 0..15 {
            r = incrementer4(r);
        }
        assert_eq!(r, _15)
    }

    #[test]
    pub fn test_incrementer8() {
        let _0 = [false, false, false, false, false, false, false, false];
        let _255 = [true, true, true, true, true, true, true, true];
        let mut r = _0;
        for _ in 0..255 {
            r = incrementer8(r);
        }
        assert_eq!(r, _255)
    }

    #[test]
    pub fn test_incrementer16() {
        let _0 = [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ];
        let _65535 = [
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true,
        ];
        let mut r = _0;
        for _ in 0..65535 {
            r = incrementer16(r);
        }
        assert_eq!(r, _65535)
    }
}
