
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![warn(unused_mut)]
#![warn(unused_variables)]
#[allow(dead_code)]

use AES::settings::{Nr, Nk};
use AES::aes_internals::{subSbox, invsubSbox, shiftRows, unshiftRows, mix, invmix, AddRoundKey, KeyExpansion};

use std::io;
use std::io::prelude::*;
use std::fs::File;

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
        let KEY : u32 = 128;
        if KEY == 256 
        {
            Nk = 8;
            Nr = 14;
        }
        if KEY == 192
        {
            Nk = 6;
            Nr = 12;
        }
        if KEY == 128 
        {
            Nk = 4;
            Nr = 10;
        }
        // let mut data : Vec<u8> = std::fs::read("../10MB").unwrap();
        let mut key  : Vec<u8> = vec![0x3c, 0x4b, 0xaa, 0xf4, 0xbd, 0xfa, 0x1e, 0x3d, 0xe5, 0x78, 0x91, 0xcf, 0xde, 0x45, 0x22, 0xab, 0x3c, 0x4b, 0x6f, 0xff, 0x21, 0xa3, 0xb5, 0xed, 0x56, 0xe1, 0x1a, 0x99, 0xa0, 0xcd, 0x45, 0x66];
        let mut RoundKey : Vec<u8> = KeyExpansion(&mut key, &KEY);
        let mut f = File::open("./sample.jpg").unwrap();
        let mut data = Vec::new();
        f.read_to_end(&mut data).unwrap();
        // let mut data : Vec<u8> = vec![0x1,0x2, 0x4, 0x5, 0x6];
        println!("{:0x?}", data.len());
        // this is PKCS#5 padding
        let mut numOfElem = data.len() as u32;
        let padded = (((numOfElem as f32)/(16.0)).ceil() as u32 ) * (16);
        let paddingElements = (padded - numOfElem) as u32;
        while padded - numOfElem > 0 {
            data.push(paddingElements as u8);
            numOfElem = data.len() as u32;
        }

        let paddedLength : u32 = data.len() as u32;

        // println!("{:0x?}", &mut data[0..(paddedLength - paddingElements) as usize]);
        println!("{:0x?}", data.len());
        // println!("{}", data.len());
        for i in (0..data.len()).step_by(16) {
            Encrypt(&mut data[i..i+16], &mut RoundKey);
        }

        println!("------------------------------------------");
        println!("------------------------------------------");
        println!("{:0x?}", data.len());

        let mut file = File::create("encrypted.jpg").unwrap();
        file.write_all(&mut data[0..(paddedLength - paddingElements) as usize]);
        println!("{:0x?}", (&mut data[0..(paddedLength - paddingElements) as usize]).len());

        println!("-----------------------------------------------");
        println!("-----------------------------------------------");
        println!("{:0x?}", data.len());

        for i in (0..data.len()).step_by(16) {
            Decrypt(&mut data[i..i+16], &mut RoundKey);
        }

        let mut file2 = File::create("decrypted.jpg").unwrap();
        file2.write_all(&mut data[0..(paddedLength - paddingElements) as usize]);
        println!("{:0x?}", (&mut data[0..(paddedLength - paddingElements) as usize]).len());

    }
}



