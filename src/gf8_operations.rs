#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![warn(unused_mut)]
#![warn(unused_variables)]
#[allow(dead_code)]

/*
 * Multiplication in GF(2^8)
 * http://en.wikipedia.org/wiki/Finite_field_arithmetic
 * Irreducible polynomial m(x) = x8 + x4 + x3 + x + 1
 */

pub fn g8mult(_x : &u8, _y: &u8) -> u8 {
    let mut a = _x.clone();
    let mut b = _y.clone();

    let mut p : u8 = 0;
    let mut overflow_check : u8 = 0;

    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        overflow_check = a & 0x80;      // if true it would overflow in the next step
        a <<= 1;
        if overflow_check != 0 {
            a ^= 0x1b;                  // subtract from it the Irreducible polynomial
        }
        b >>= 1;
    }
    p
}

/*
 * Addition in GF(2^8)
 * http://en.wikipedia.org/wiki/Finite_field_arithmetic
 */

pub fn g8add(_x : &u8, _y: &u8) -> u8 {
    let a = _x.clone();
    let b = _y.clone();

    let p : u8 = a ^ b;

    p
}

/*
 * Subtraction in GF(2^8)
 * http://en.wikipedia.org/wiki/Finite_field_arithmetic
 */

pub fn g8sub(_x: &u8, _y: &u8) -> u8 {
    let a = _x.clone();
    let b = _y.clone();

    let p : u8 = a ^ b;

    p
}

