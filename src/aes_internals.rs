#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![warn(unused_mut)]
#![warn(unused_variables)]
#[allow(dead_code)]


use crate::gf8_operations::{g8mult, g8add};
use crate::tables::{SBOX, INVSBOX, Rcon, MixColumn, invMixColumn};
use crate::settings::{Nb, Nr, Nk, CBC, ECB};


// unsafe keyword because we are using Nr, Nb, Nk as global functions
// these global variable would be deleted soon

pub unsafe fn subSbox(data: &mut [u8]) {
    for byte in data.iter_mut() {
        *byte = SBOX[*byte as usize];
    }
}

pub unsafe fn invsubSbox(data: &mut [u8]) {
    for byte in data.iter_mut() {
        *byte = INVSBOX[*byte as usize];
    }
}



pub unsafe fn shiftRows(data: &mut [u8]) {

    // second row --> 1 shift to left
    // second row indices is 1-5-9-12
    {
        let temp = data[1];
        data[1]  = data[5];
        data[5]  = data[9];
        data[9]  = data[13];
        data[13] = temp;
    }

    // third row --> 2 shifts to left
    // third row indices is 2-6-10-14
    {
        let temp1 = data[2];
        let temp2 = data[6];
        data[2] = data[10];
        data[6] = data[14];

        data[10] = temp1;
        data[14] = temp2;
    }

    // fourth row --> 3 shifts to left
    // fourth row indices is 3-7-11-15
    {
        let temp = data[15];
        data[15] = data[11];
        data[11] = data[7];
        data[7]  = data[3];
        data[3]  = temp;
    }
}


pub unsafe fn unshiftRows(data: &mut [u8]) {

    // second row --> 1 shifts to right
    {
        let temp = data[13];
        data[13] = data[9];
        data[9] = data[5];
        data[5] = data[1];
        data[1] = temp;
    }

    // third row --> 2 shifts to right
    {
        let temp1 = data[14];
        let temp2 = data[10];
        data[14] = data[6];
        data[10] = data[2];

        data[6] = temp1;
        data[2] = temp2;
    }

    // fourth row --> 3 shits to right
    {
        let temp = data[3];
        data[3] = data[7];
        data[7] = data[11];
        data[11] = data[15];
        data[15] = temp;
    }
}

pub unsafe fn mix(data: &mut [u8]) {
    for word in 0..4 
    {
        let columnOfstate = data[word*4 .. word*4+4].to_vec().clone();

        data[word * 4 + 0] = g8add(&g8mult(&columnOfstate[0], &MixColumn[0][0]) , &g8add(&g8mult(&columnOfstate[1], &MixColumn[0][1]) 
                           , &g8add(&g8mult(&columnOfstate[2], &MixColumn[0][2]) , &g8mult(&columnOfstate[3], &MixColumn[0][3]))));
                             

        data[word * 4 + 1] = g8add(&g8mult(&columnOfstate[0], &MixColumn[1][0]) , &g8add(&g8mult(&columnOfstate[1], &MixColumn[1][1]) 
                           , &g8add(&g8mult(&columnOfstate[2], &MixColumn[1][2]) , &g8mult(&columnOfstate[3], &MixColumn[1][3]))));
                             

        data[word * 4 + 2] = g8add(&g8mult(&columnOfstate[0], &MixColumn[2][0]) , &g8add(&g8mult(&columnOfstate[1], &MixColumn[2][1]) 
                           , &g8add(&g8mult(&columnOfstate[2], &MixColumn[2][2]) , &g8mult(&columnOfstate[3], &MixColumn[2][3]))));
                             

        data[word * 4 + 3] = g8add(&g8mult(&columnOfstate[0], &MixColumn[3][0]) , &g8add(&g8mult(&columnOfstate[1], &MixColumn[3][1]) 
                           , &g8add(&g8mult(&columnOfstate[2], &MixColumn[3][2]) , &g8mult(&columnOfstate[3], &MixColumn[3][3]))));
    }
}

pub unsafe fn invmix(data: &mut [u8]) {
    for word in 0..4 
    {
        let columnOfstate = data[word*4 .. word*4+4].to_vec().clone();
        
        data[word * 4 + 0] = g8add(&g8mult(&columnOfstate[0], &invMixColumn[0][0]) , &g8add(&g8mult(&columnOfstate[1], &invMixColumn[0][1]) 
                           , &g8add(&g8mult(&columnOfstate[2], &invMixColumn[0][2]) , &g8mult(&columnOfstate[3], &invMixColumn[0][3]))));
                             
    
        data[word * 4 + 1] = g8add(&g8mult(&columnOfstate[0], &invMixColumn[1][0]) , &g8add(&g8mult(&columnOfstate[1], &invMixColumn[1][1]) 
                           , &g8add(&g8mult(&columnOfstate[2], &invMixColumn[1][2]) , &g8mult(&columnOfstate[3], &invMixColumn[1][3]))));
                             
    
        data[word * 4 + 2] = g8add(&g8mult(&columnOfstate[0], &invMixColumn[2][0]) , &g8add(&g8mult(&columnOfstate[1], &invMixColumn[2][1]) 
                           , &g8add(&g8mult(&columnOfstate[2], &invMixColumn[2][2]) , &g8mult(&columnOfstate[3], &invMixColumn[2][3]))));
                             
    
        data[word * 4 + 3] = g8add(&g8mult(&columnOfstate[0], &invMixColumn[3][0]) , &g8add(&g8mult(&columnOfstate[1], &invMixColumn[3][1]) 
                           , &g8add(&g8mult(&columnOfstate[2], &invMixColumn[3][2]) , &g8mult(&columnOfstate[3], &invMixColumn[3][3]))));
    }
}

pub unsafe fn AddRoundKey(data : &mut [u8], RoundKey : & [u8], round : & u8) {
    for i in 0..4*Nb {
        data[i as usize] = data[i as usize] ^ RoundKey[((round * Nb * 4) + i ) as usize];
    }
}

pub unsafe fn KeyExpansion(key : &[u8], KEY : &u32) -> Vec<u8> {

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
        if i % Nk == 0 // at every Nk word ... every 4 words in 128 bit and 6 words in 192 bit and 8 words in 256 bit
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

        if *KEY == 256 
        {
            if i % Nk == 4 
            {
                temp[0] = SBOX[temp[0] as usize];
                temp[1] = SBOX[temp[1] as usize];
                temp[2] = SBOX[temp[2] as usize];
                temp[3] = SBOX[temp[3] as usize];
            }
        }

        // the word above it
        let k = (i - Nk) * 4;

        RoundKey[((i * 4) + 0) as usize] = RoundKey[(k + 0) as usize] ^ temp[0];
        RoundKey[((i * 4) + 1) as usize] = RoundKey[(k + 1) as usize] ^ temp[1];
        RoundKey[((i * 4) + 2) as usize] = RoundKey[(k + 2) as usize] ^ temp[2];
        RoundKey[((i * 4) + 3) as usize] = RoundKey[(k + 3) as usize] ^ temp[3];
    }

    RoundKey
}


