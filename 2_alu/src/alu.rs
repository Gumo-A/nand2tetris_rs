use crate::adders;
use logic_gates::Arr16;
use logic_gates::basic_gates as bg;
use logic_gates::multibit_basic_gates as mbg;
use logic_gates::multiway_basic_gates as mwg;

const ARR16_0: Arr16 = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false,
];

pub fn alu(
    x: Arr16,
    y: Arr16,
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> (Arr16, bool, bool) {
    let x = mbg::mux16(x, ARR16_0, zx);
    let x = mbg::mux16(x, mbg::not16(x), nx);

    let y = mbg::mux16(y, ARR16_0, zy);
    let y = mbg::mux16(y, mbg::not16(y), ny);

    let result = mbg::mux16(adders::adder16(x, y), mbg::and16(x, y), f);
    let result = mbg::mux16(mbg::not16(result), result, no);

    let zr = bg::not(mwg::or16way(result));
    let ng = result[0];

    (result, zr, ng)
}
