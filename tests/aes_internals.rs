#![allow(non_snake_case)]
#![warn(non_upper_case_globals)]
#![allow(non_snake_case)]
#![warn(unused_mut)]
#![warn(unused_variables)]
#[allow(dead_code)]


use AES::aes_internals::{subSbox, invsubSbox, shiftRows, unshiftRows, mix, invmix, AddRoundKey};

#[test]
fn test_sbox() {
    unsafe {
        let mut inputSbox  : Vec<u8> = vec![0x6a, 0x84, 0x86, 0x7c, 0xd7, 0x7e, 0x12, 0xad, 0x07, 0xea, 0x1b, 0xe8, 0x95, 0xc5, 0x3f, 0xa3];
        let mut outputSbox : Vec<u8> = vec![0x02, 0x5f, 0x44, 0x10, 0x0e, 0xf3, 0xc9, 0x95, 0xc5, 0x87, 0xaf, 0x9b, 0x2a, 0xa6, 0x75, 0x0a];

        // we will pass it through sbox function and mutate it

        subSbox(&mut inputSbox);
        assert_eq!(inputSbox, outputSbox);

        invsubSbox(&mut inputSbox);         // to get it back to its original

        // inverse the outputSbox to get inputSbox

        invsubSbox(&mut outputSbox);
        assert_eq!(inputSbox, outputSbox);
    }
}

#[test]
fn test_shift() {
    unsafe {
        let mut inputShiftRows : Vec<u8> = vec![0x02, 0x5f, 0x44, 0x10, 0x0e, 0xf3, 0xc9, 0x95, 0xc5, 0x87, 0xaf, 0x9b, 0x2a, 0xa6, 0x75, 0x0a];
        let mut outputShiftRows: Vec<u8> = vec![0x02, 0xf3, 0xaf, 0x0a, 0x0e, 0x87, 0x75, 0x10, 0xc5, 0xa6, 0x44, 0x95, 0x2a, 0x5f, 0xc9, 0x9b];

        // let's shift inputShitRows 

        shiftRows(&mut inputShiftRows);     // the inputShiftRows has been mutated so to use again we need to un-do what we have done
        assert_eq!(inputShiftRows, outputShiftRows);

        unshiftRows(&mut inputShiftRows);

        // inverse of shift rows
        unshiftRows(&mut outputShiftRows);
        assert_eq!(inputShiftRows, outputShiftRows);
    }
}

#[test]
fn test_mix() {
    unsafe {
        let mut inputMixCol : Vec<u8> = vec![0x02, 0xf3, 0xaf, 0x0a, 0x0e, 0x87, 0x75, 0x10, 0xc5, 0xa6, 0x44, 0x95, 0x2a, 0x5f, 0xc9, 0x9b];
        let mut outputMixCol: Vec<u8> = vec![0xaf, 0x1f, 0xaa, 0x4e, 0xeb, 0x94, 0x53, 0xc0, 0xb1, 0xcb, 0x4f, 0x87, 0xe7, 0x4f, 0x4a, 0xc5];

        // mix the columns and mutate the original array
        mix(&mut inputMixCol);
        assert_eq!(inputMixCol, outputMixCol);

        invmix(&mut inputMixCol);

        // inverse the mix columns
        invmix(&mut outputMixCol);
        assert_eq!(inputMixCol, outputMixCol);
    }
}


#[test]
fn key_addition() {
    unsafe {
        // this is just XOR-ing

        let mut data       : Vec<u8> = vec![0xaf, 0x1f, 0xaa, 0x4e, 0xeb, 0x94, 0x53, 0xc0, 0xb1, 0xcb, 0x4f, 0x87, 0xe7, 0x4f, 0x4a, 0xc5];
        let mut key        : Vec<u8> = vec![0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63];
        let expectedOutput : Vec<u8> = vec![0xcd, 0x7c, 0xc9, 0x2d, 0x89, 0xf7, 0x30, 0xa3, 0xd3, 0xa8, 0x2c, 0xe4, 0x85, 0x2c, 0x29, 0xa6];
        AddRoundKey(&mut data, &mut key, &0);
        assert_eq!(data, expectedOutput); 
    }
}