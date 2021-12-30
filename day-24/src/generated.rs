#[inline]
pub fn prog(prog_idx: usize, w: i64, z: i64) -> i64 {
    let input0 = w;
    let input1 = z;
    match prog_idx {
        0 => {
            ((input1
                * ((if (((input1 % 26) + 11) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + 11) != input0) {
                    (input0 + 3)
                } else {
                    0
                }))
        }
        1 => {
            ((input1
                * ((if (((input1 % 26) + 14) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + 14) != input0) {
                    (input0 + 7)
                } else {
                    0
                }))
        }
        2 => {
            ((input1
                * ((if (((input1 % 26) + 13) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + 13) != input0) {
                    (input0 + 1)
                } else {
                    0
                }))
        }
        3 => {
            (((input1 / 26)
                * ((if (((input1 % 26) + -4) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + -4) != input0) {
                    (input0 + 6)
                } else {
                    0
                }))
        }
        4 => {
            ((input1
                * ((if (((input1 % 26) + 11) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + 11) != input0) {
                    (input0 + 14)
                } else {
                    0
                }))
        }
        5 => {
            ((input1
                * ((if (((input1 % 26) + 10) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + 10) != input0) {
                    (input0 + 7)
                } else {
                    0
                }))
        }
        6 => {
            (((input1 / 26)
                * ((if (((input1 % 26) + -4) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + -4) != input0) {
                    (input0 + 9)
                } else {
                    0
                }))
        }
        7 => {
            (((input1 / 26)
                * ((if (((input1 % 26) + -12) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + -12) != input0) {
                    (input0 + 9)
                } else {
                    0
                }))
        }
        8 => {
            ((input1
                * ((if (((input1 % 26) + 10) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + 10) != input0) {
                    (input0 + 6)
                } else {
                    0
                }))
        }
        9 => {
            (((input1 / 26)
                * ((if (((input1 % 26) + -11) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + -11) != input0) {
                    (input0 + 4)
                } else {
                    0
                }))
        }
        10 => {
            ((input1
                * ((if (((input1 % 26) + 12) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + 12) != input0) {
                    input0
                } else {
                    0
                }))
        }
        11 => {
            (((input1 / 26)
                * ((if (((input1 % 26) + -1) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + -1) != input0) {
                    (input0 + 7)
                } else {
                    0
                }))
        }
        12 => {
            (((input1 / 26) * ((if ((input1 % 26) != input0) { 25 } else { 0 }) + 1))
                + (if ((input1 % 26) != input0) {
                    (input0 + 12)
                } else {
                    0
                }))
        }
        13 => {
            (((input1 / 26)
                * ((if (((input1 % 26) + -11) != input0) {
                    25
                } else {
                    0
                }) + 1))
                + (if (((input1 % 26) + -11) != input0) {
                    (input0 + 1)
                } else {
                    0
                }))
        }
        _ => unimplemented!(),
    }
}
