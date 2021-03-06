#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![warn(unused_mut)]
#![warn(unused_variables)]
#[allow(dead_code)]

// Number of 32-bit words in each state. This is constant in AES.

pub static Nb : u8 = 4;


// The default key size would be 128 bit ... the user could toggle a ternary variable
// to change from 128 bit to 192 bit to 256 bit

pub static mut Nk: u8 = 4;      // number of 32 bit words in a key
pub static mut Nr: u8 = 10;     // number of rounds in 128 bit key size



// AES mode of operations and key size variable

// 128 bit is the default
// pub static mut KEY : u32 = 128;

// ECB is the default mode

pub static mut ECB : bool = true;

pub static mut CBC : bool = false;

