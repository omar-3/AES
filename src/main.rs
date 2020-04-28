#![allow(non_snake_case)]

use AES::gf8_operations::{g8mult, g8add, g8sub};
use AES::tables::{SBOX, INVSBOX, Rcon};
use AES::settings::{Nb, Nr, Nk};

unsafe fn KeyExpansion(key : &[u8]) -> Vec<u8> {
    let mut RoundKey : Vec<u8> = vec![0; ((Nb * (Nr + 1)) * 4) as usize];
    let mut temp : Vec<u8> = vec![0; 4];
    for i in 0..Nk 
    {
        RoundKey[((i * 4) + 0) as usize] = key[((i * 4) + 0) as usize];
        RoundKey[((i * 4) + 1) as usize] = key[((i * 4) + 1) as usize];
        RoundKey[((i * 4) + 2) as usize] = key[((i * 4) + 2) as usize];
        RoundKey[((i * 4) + 3) as usize] = key[((i * 4) + 3) as usize];
    }

    for i in Nk..(Nb * (Nr + 1)) 
    {
        // The keys of the last word
        {
            let k : u8 = (i - 1) * 4;
            temp[0] = RoundKey[(k + 0) as usize];
            temp[1] = RoundKey[(k + 1) as usize];
            temp[2] = RoundKey[(k + 2) as usize];
            temp[3] = RoundKey[(k + 3) as usize];
        }
        
        // if we need to use the g function
        if (i % Nk == 0) // at every Nk word ... every 4 words in 128 bit and 6 words in 192 bit and 8 words in 256 bit
        {
            // rotate the 4 words
            let tobeRotated = temp[0];
            temp[0] = temp[1];
            temp[1] = temp[2];
            temp[2] = temp[3];
            temp[3] = tobeRotated;

            // sbox substitution
            temp[0] = SBOX[temp[0] as usize];
            temp[1] = SBOX[temp[1] as usize];
            temp[2] = SBOX[temp[2] as usize];
            temp[3] = SBOX[temp[3] as usize];

            // add Rcon component 
            temp[0] = temp[0] ^ Rcon[((i/Nk) - 1) as usize];
        }

        let k = (i - Nk) * 4;
        RoundKey[((i * 4) + 0) as usize] = RoundKey[(k + 0) as usize] ^ temp[0];
        RoundKey[((i * 4) + 1) as usize] = RoundKey[(k + 1) as usize] ^ temp[1];
        RoundKey[((i * 4) + 2) as usize] = RoundKey[(k + 2) as usize] ^ temp[2];
        RoundKey[((i * 4) + 3) as usize] = RoundKey[(k + 3) as usize] ^ temp[3];
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



