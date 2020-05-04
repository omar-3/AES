
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![warn(unused_mut)]
#![warn(unused_variables)]
#[allow(dead_code)]


use AES::settings::{Nr};
use AES::aes_internals::{subSbox, invsubSbox, shiftRows, unshiftRows, mix, invmix, AddRoundKey, KeyExpansion};

unsafe fn Encrypt(state : &mut [u8], RoundKey : &mut [u8]) {
    let mut round = 0;

    AddRoundKey(state , & RoundKey, & round);
    loop {
        round = round + 1;
        subSbox(state);
        shiftRows(state);
        if round == Nr {
            break;
        }
        mix(state);
        AddRoundKey(state, & RoundKey, & round);
    }

    AddRoundKey(state, & RoundKey, & round);
}

unsafe fn Decrypt(state: &mut [u8], RoundKey : &mut [u8]) {
    let mut round = Nr;

    AddRoundKey(state , & RoundKey, & round);
    loop {
        round = round - 1;
        unshiftRows(state);
        invsubSbox(state);
        AddRoundKey(state , & RoundKey, & round);
        if round == 0 {
            break;
        }
        invmix(state);
    }
}


fn main() {
    unsafe {
        let mut data : Vec<u8> = vec![0x58, 0xc8, 0xe0, 0x0b, 0x26, 0x31, 0x68, 0x6d, 0x54, 0xea, 0xb8, 0x4b, 0x91, 0xf0, 0xac, 0xa1];
        let mut key  : Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut RoundKey : Vec<u8> = KeyExpansion(&mut key);
        Encrypt(&mut data, &mut RoundKey);
        println!("{:02x?}", data);                          // data array has been mutated and encrypted
        Decrypt(&mut data, &mut RoundKey);
        println!("{:02x?}", data);                          // data array should be as we have entered
    }
}



