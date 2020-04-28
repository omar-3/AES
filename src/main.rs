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

        // the word above it
        let k = (i - Nk) * 4;

        RoundKey[((i * 4) + 0) as usize] = RoundKey[(k + 0) as usize] ^ temp[0];
        RoundKey[((i * 4) + 1) as usize] = RoundKey[(k + 1) as usize] ^ temp[1];
        RoundKey[((i * 4) + 2) as usize] = RoundKey[(k + 2) as usize] ^ temp[2];
        RoundKey[((i * 4) + 3) as usize] = RoundKey[(k + 3) as usize] ^ temp[3];
    }

    RoundKey
}

unsafe fn AddRoundKey(data : &mut [u8], RoundKey : & [u8], round : u8) {
    for i in 0..4*Nb {
        data[i as usize] = data[i as usize] ^ RoundKey[((round * Nb * 4) + i ) as usize];
    }
}

unsafe fn subSbox(data: &mut [u8]) {
    for byte in data.iter_mut() {
        *byte = SBOX[*byte as usize];
    }
}

unsafe fn invsubSbox(data: &mut [u8]) {
    for byte in data.iter_mut() {
        *byte = INVSBOX[*byte as usize];
    }
}

unsafe fn shiftRows(data: &mut [u8]) {

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


unsafe fn unshiftRows(data: &mut [u8]) {

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


fn main() {
    unsafe {
        let mut key : Vec<u8> = vec![52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60];
        let mut data : Vec<u8> = vec![52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60,
                                      52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60,
                                      52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60,
                                      52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60,
                                      52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60,
                                      52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60,
                                      52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60,
                                      52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60,
                                      52,16,25,36,54,62,51,65,53,26,54,62,54,76,95,60];
        println!("{:?}", key);
        println!("---------------------------------------------");
        shiftRows(&mut key);
        println!("{:?}", key);
        println!("---------------------------------------------");
        unshiftRows(&mut key);
        println!("{:?}", key);
        println!("---------------------------------------------");

    }
}



