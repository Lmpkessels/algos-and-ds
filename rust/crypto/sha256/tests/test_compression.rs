use padd_pars::{
    big_endian_padd, big_endian_pars
};

use sha256::schedule::schedule;

use sha256::compression::compress;

#[test]
fn test_empty_string_compute_digested_array() {
    let msg = b"";
    let padding = big_endian_padd(msg);
    let parsing = big_endian_pars(padding);
    let scheduled = schedule(parsing);

    let result = compress(scheduled);
    let expected = [
        0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, 
        0x27ae41e4, 0x649b934c, 0xa495991b, 0x7852b855,
    ];

    assert_eq!((result), (expected));
}

#[test]
fn use_one_word_compute_digested_array() {
    let msg = b"abc";
    let padding = big_endian_padd(msg);
    let parsing = big_endian_pars(padding);
    let scheduled = schedule(parsing);

    let result = compress(scheduled);
    let expected = [
        0xba7816bf, 0x8f01cfea, 0x414140de, 0x5dae2223, 
        0xb00361a3, 0x96177a9c, 0xb410ff61, 0xf20015ad,
    ];

    assert_eq!((result), (expected));
}

#[test]
fn expanding_in_second_block_compute_digested_array() {
    let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    let padding = big_endian_padd(msg);
    let parsing = big_endian_pars(padding);
    let scheduled = schedule(parsing);

    let result = compress(scheduled);
    let expected = [
        0x248d6a61, 0xd20638b8, 0xe5c02693, 0x0c3e6039, 
        0xa33ce459, 0x64ff2167, 0xf6ecedd4, 0x19db06c1,
    ];

    assert_eq!((result), (expected));
}

#[test]
fn test_one_million_a_compute_digested_array() {
    let msg = b"a".repeat(1_000_000);
    let padding = big_endian_padd(&msg);
    let parsing = big_endian_pars(padding);
    let scheduled = schedule(parsing);

    let result = compress(scheduled);
    let expected = [
        0xcdc76e5c, 0x9914fb92, 0x81a1c7e2, 0x84d73e67, 
        0xf1809a48, 0xa497200e, 0x046d39cc, 0xc7112cd0,
    ];

    assert_eq!((result), (expected));
}