#![allow(non_snake_case)]

use AES::gf8_operations::{g8mult, g8add, g8sub};
use AES::tables::{SBOX, INVSBOX, Rcon};
use AES::settings::{Nb, Nr, Nk};

unsafe fn KeyExpansion(key : &[u8]) -> Vec<u8> {
    let mut RoundKey : Vec<u8> = vec![0; ((Nb * (Nr + 1)) * 4) as usize];
    let mut temp : Vec<u8> = vec![0; 4];
    for i in 0..Nk {
        RoundKey[((i * 4) + 0) as usize] = key[((i * 4) + 0) as usize];
        RoundKey[((i * 4) + 1) as usize] = key[((i * 4) + 1) as usize];
        RoundKey[((i * 4) + 2) as usize] = key[((i * 4) + 2) as usize];
        RoundKey[((i * 4) + 3) as usize] = key[((i * 4) + 3) as usize];
    }

    for i in Nk..(Nb * (Nr + 1)) {
        temp[0] = 1
    }
    
    

    RoundKey
}

fn main() {
    unsafe {
        let mut bytes : Vec<u8> = vec![52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60];
        println!("{:?}", bytes);
        println!("{}", KeyExpansion(&mut bytes).len());
        
    }
}



