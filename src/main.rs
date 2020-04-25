#![allow(non_snake_case)]
#![warn(unused_assignments)]
#![allow(overflowing_literals)]

// use AES::tables::{SBOX, INVSBOX};


fn g8mult(a : &mut u8, b: &mut u8) -> u8 {
    let mut p : u8 = 0;
    let mut hbs : u8 = 0;

    for _ in 0..8 {
        if *b & 1 != 0 {
            p ^= *a;
        }
        hbs = *a & 0x80;
        *a <<= 1;
        if hbs != 0 {
            *a ^= 0x1b;
        }
        *b >>= 1;
    }

    p
}

fn main() {

    let mut a : u8 = 0xc2;
    let mut b : u8 = 0x2f;
    let c : u8 = g8mult(&mut a, &mut b);
    println!("{}", c);
    println!("{}",b);

}



