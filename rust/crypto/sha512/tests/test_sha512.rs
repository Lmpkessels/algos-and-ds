use padd_pars::{
    big_endian_padd64, big_endian_pars64        
};

use sha512::schedule::schedule;

use sha512::hash::sha512;

#[test]
fn sha512_computes_empty_string() {
    let msg = b"";
    let padded = big_endian_padd64(msg);
    let parsed = big_endian_pars64(padded);
    let scheduled = schedule(&parsed);
    
    let result = sha512(&scheduled);
    let expected = [
        0xcf83e1357eefb8bd, 
        0xf1542850d66d8007, 
        0xd620e4050b5715dc,
        0x83f4a921d36ce9ce,
        0x47d0d13c5d85f2b0,
        0xff8318d2877eec2f, 
        0x63b931bd47417a81,
        0xa538327af927da3e
    ];
    
    assert_eq!((result), (expected));
}

#[test]
fn sha512_computes_abc() {
    let msg = b"abc";
    let padded = big_endian_padd64(msg);
    let parsed = big_endian_pars64(padded);
    let scheduled = schedule(&parsed);
    
    let result = sha512(&scheduled);
    let expected = [
        0xddaf35a193617aba,
        0xcc417349ae204131,
        0x12e6fa4e89a97ea2, 
        0x0a9eeee64b55d39a,
        0x2192992a274fc1a8, 
        0x36ba3c23a3feebbd,
        0x454d4423643ce80e,
        0x2a9ac94fa54ca49f
    ];

    assert_eq!((result), (expected));
}

#[test]
fn sha512_computes_brown_fox() {
    let msg = b"The quick brown fox jumps over the lazy dog";
    let padded = big_endian_padd64(msg);
    let parsed = big_endian_pars64(padded);
    let scheduled = schedule(&parsed);
    
    let result = sha512(&scheduled);
    let expected = [
        0x07e547d9586f6a73, 
        0xf73fbac0435ed769,
        0x51218fb7d0c8d788, 
        0xa309d785436bbb64,
        0x2e93a252a954f239,
        0x12547d1e8a3b5ed6,
        0xe1bfd7097821233f,
        0xa0538f3db854fee6
    ];

    assert_eq!((result), (expected));
}

#[test]
fn sha512_computes_1_000_000_as() {
    let msg = b"a".repeat(1_000_000);
    let padded = big_endian_padd64(&msg);
    let parsed = big_endian_pars64(padded);
    let scheduled = schedule(&parsed);
    
    let result = sha512(&scheduled);
    let expected = [
        0xe718483d0ce76964,
        0x4e2e42c7bc15b463,
        0x8e1f98b13b204428,
        0x5632a803afa973eb,
        0xde0ff244877ea60a,
        0x4cb0432ce577c31b,
        0xeb009c5c2c49aa2e,
        0x4eadb217ad8cc09b
    ];

    assert_eq!((result), (expected));
}