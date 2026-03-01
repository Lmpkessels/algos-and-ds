use padd_pars::{
    little_endian_padd, little_endian_pars
};

use ripemd160::hash::ripemd160;

#[test]
fn ripemd160_empty_string() {
    let msg = b"";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    
    let result = ripemd160(parsed);
    let expected = [
        0x9c1185a5, 0xc5e9fc54, 0x61280897, 0x7ee8f548, 0xb2258d31
    ];

    assert_eq!((result), (expected));
}

#[test]
fn ripemd160_single_a() {
    let msg = b"a";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);

    let result = ripemd160(parsed);
    let expected = [
        0x0bdc9d2d, 0x256b3ee9, 0xdaae347b, 0xe6f4dc83, 0x5a467ffe
    ];

    assert_eq!((result), (expected));
}

#[test]
fn ripemd160_abc() {
    let msg = b"abc";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);

    let result = ripemd160(parsed);
    let expected = [
        0x8eb208f7, 0xe05d987a, 0x9b044a8e, 0x98c6b087, 0xf15a0bfc
    ];

    assert_eq!((result), (expected));
}

#[test]
fn ripemd160_long_message() {
    let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);

    let result = ripemd160(parsed);
    let expected = [
        0x12a05338, 0x4a9c0c88, 0xe405a06c, 0x27dcf49a, 0xda62eb2b
    ];

    assert_eq!((result), (expected));
}  

#[test]
fn ripemd160_1million_as() {
    let msg = b"a".repeat(1_000_000);
    let padded = little_endian_padd(&msg);
    let parsed = little_endian_pars(padded);
    
    let result = ripemd160(parsed);
    let expected = [
        0x52783243, 0xc1697bdb, 0xe16d37f9, 0x7f68f083, 0x25dc1528
    ];

    assert_eq!((result), (expected));
}