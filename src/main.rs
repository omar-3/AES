#![allow(non_snake_case)]

use AES::{g8mult, sbox, invsbox};

fn main() {
    let khaled = g8mult!(57, 13);
    println!("{}", khaled);

    let mut z : Vec<u8> = vec![0,0,0,0];


    z[0] = g8mult!(sbox![0], invsbox![0]);
    z[1] = g8mult!(sbox![1], invsbox![1]);
    z[2] = g8mult!(sbox![2], invsbox![2]);
    z[3] = g8mult!(sbox![3], invsbox![3]);

    println!("{:?}", z);
    let omar = sbox!(0xc2);
    println!("{}", omar);
    let shh = invsbox!(10);
    println!("{}", shh);   
}