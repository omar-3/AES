#![allow(non_snake_case)]

use AES::tables::{SBOX, INVSBOX};
use AES::gf8_operations::{g8mult, g8add, g8sub};
use std::fs;

fn main() {
    let mut bytes = fs::read("./main.c").unwrap();
    
}



