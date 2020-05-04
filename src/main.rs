
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

        // let mut key : Vec<u8> = vec![0x02, 0x5f, 0x44, 0x10, 0x0e, 0xf3, 0xc9, 0x95, 0xc5, 0x87, 0xaf, 0x9b, 0x2a, 0xa6, 0x75, 0x0a];
        // println!("{:02x?}", key);
        // shiftRows(&mut key);
        // println!("{:02x?}", key);
        // unshiftRows(&mut key);
        // println!("{:02x?}", key);

        // let mut inputMixCol : Vec<u8> = vec![0x02, 0xf3, 0xaf, 0x0a, 0x0e, 0x87, 0x75, 0x10, 0xc5, 0xa6, 0x44, 0x95, 0x2a, 0x5f, 0xc9, 0x9b];
        // mix(&mut inputMixCol);
        // println!("{:02x?}", inputMixCol);
        // invmix(&mut inputMixCol);
        // println!("{:02x?}", inputMixCol);

        // let mut data : Vec<u8> = vec![0xaf, 0x1f, 0xaa, 0x4e, 0xeb, 0x94, 0x53, 0xc0, 0xb1, 0xcb, 0x4f, 0x87, 0xe7, 0x4f, 0x4a, 0xc5];
        // let mut key  : Vec<u8> = vec![0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63];
        // AddRoundKey(&mut data, &mut key, 0);
        // println!("{:02x?}", data);

        let mut data : Vec<u8> = vec![0x58, 0xc8, 0xe0, 0x0b, 0x26, 0x31, 0x68, 0x6d, 0x54, 0xea, 0xb8, 0x4b, 0x91, 0xf0, 0xac, 0xa1];
        let mut key  : Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut RoundKey : Vec<u8> = KeyExpansion(&mut key);
        Encrypt(&mut data, &mut RoundKey);
        println!("{:02x?}", data);                          // data array has been mutated and encrypted
        Decrypt(&mut data, &mut RoundKey);
        println!("{:02x?}", data);                          // data array should be as we have entered
    }
}



