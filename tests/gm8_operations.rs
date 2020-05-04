#![allow(non_snake_case)]
#![warn(non_upper_case_globals)]
#![allow(non_snake_case)]
#![warn(unused_mut)]
#![warn(unused_variables)]
#![warn(dead_code)]

use AES::gf8_operations::{g8mult, g8add, g8sub};


#[test]
fn gf8multiplication() {
    
    // still in GF8 bounds, so this is just a simple multiplication
    assert_eq!(g8mult(&5, &6), 30);

    // Paar textbook page 99
    assert_eq!(g8mult(&0xc2, &0x2f), 1);

    // From wikipedia article https://en.wikipedia.org/wiki/Finite_field_arithmetic#Rijndael's_(AES)_finite_field
    assert_eq!(g8mult(&0x53, &0xca), 1);

    // unfortunately I can't find a reliable source in which there are more examples for GF8 multiplication
    // but what we can do is to use mixcolumn matrix numbers and the inverse and surely we know the result
    // we will take the first row of mixcolumn and first column of the inverse and multiply them together element-wise
    // and sum all the elements of the result vector, it should be 1
    
    let mut first_row_mix_column: Vec<u8> = Vec::new();
    first_row_mix_column.push(0x02);
    first_row_mix_column.push(0x03);
    first_row_mix_column.push(0x01);
    first_row_mix_column.push(0x01);

    let mut first_column_inv_mix_column: Vec<u8> = Vec::new();
    first_column_inv_mix_column.push(0x0e);
    first_column_inv_mix_column.push(0x09);
    first_column_inv_mix_column.push(0x0d);
    first_column_inv_mix_column.push(0x0b);

    let mut element_wise_mult : Vec<u8> = vec![0,0,0,0];
    element_wise_mult[0] = g8mult(&first_row_mix_column[0], &first_column_inv_mix_column[0]);
    element_wise_mult[1] = g8mult(&first_row_mix_column[1], &first_column_inv_mix_column[1]);
    element_wise_mult[2] = g8mult(&first_row_mix_column[2], &first_column_inv_mix_column[2]);
    element_wise_mult[3] = g8mult(&first_row_mix_column[3], &first_column_inv_mix_column[3]);

    let sum = g8add(&element_wise_mult[0], &g8add(&element_wise_mult[1], &g8add(&element_wise_mult[2], &element_wise_mult[3])));
    
    assert_eq!(sum, 1);
}

/*
 * addition and subtraction in GF8 is just bit-wise XOR-ing for the two numbers
 */
#[test]
fn gf8add_sub() {

    /*
     *  5 ==> 0000_0101
     *       XOR
     *  6 ==> 0000_0110
     *  --------------- 
     *  3 ==> 0000_0011
     */

    assert_eq!(g8add(&5, &6), 3);
    assert_eq!(g8sub(&5, &6), 3);

    assert_eq!(g8add(&1, &1), 0);
    assert_eq!(g8sub(&1, &1), 0);
}