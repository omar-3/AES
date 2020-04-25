use AES::{g8mult, sbox, invsbox};

#[test]

fn gf8multiplication() {
    
    // still in GF8 bounds, so this is just multiplication
    assert_eq!(g8mult!(5, 6), 30);

    // Paar textbook page 99
    assert_eq!(g8mult!(0xc2, 0x2f), 1);

    // From wikipedia article https://en.wikipedia.org/wiki/Finite_field_arithmetic#Rijndael's_(AES)_finite_field
    assert_eq!(g8mult(0x53,0xca), 1);

    // unfortunately I can't find a reliable source in which there are more examples for GF8 multiplication
    // but what we can do is using mixcolumn matrix numbers and surely we know its result

}