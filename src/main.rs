#![allow(non_snake_case)]

use AES::{g8mult, sbox, invsbox};

fn main() {
    let khaled = g8mult!(57, 13);
    println!("{}", khaled);

    let mut row1MixColumn: Vec<u8> = Vec::new();
    row1MixColumn.push(0x02);
    row1MixColumn.push(0x03);
    row1MixColumn.push(0x01);
    row1MixColumn.push(0x01);
    println!("{}", row1MixColumn.len());
    println!("{:?}", row1MixColumn);
    let mut column1InvMixColumn: Vec<u8> = Vec::new();
    column1InvMixColumn.push(0x0e);
    column1InvMixColumn.push(0x09);
    column1InvMixColumn.push(0x0d);
    column1InvMixColumn.push(0x0b);
    println!("{}", column1InvMixColumn.len());
    println!("{:?}", column1InvMixColumn);
    let mut z : Vec<u8> = Vec::with_capacity(4);
    for ((zval, aval), bval) in z.iter_mut().zip(&row1MixColumn).zip(&column1InvMixColumn) {
        *zval = *aval + *bval;
    }
    
    println!("{:?}", z);
    let omar = sbox!(0xc2);
    println!("{}", omar);
    let shh = invsbox!(10);
    println!("{}", shh);
}