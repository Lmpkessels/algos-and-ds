use padd_pars::{
    little_endian_padd, little_endian_pars
};

use md5::hash::md5;

#[test]
fn md5_compute_empty_string() {
    let msg = b"";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let result = md5(&parsed);
    let expected = [
        0xd41d8cd9, 0x8f00b204, 0xe9800998, 0xecf8427e
    ];
    assert_eq!((expected), (result));
}

#[test]
fn md5_compute_one_a() {
    let msg = b"a";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let result = md5(&parsed);
    let expected = [
        0x0cc175b9, 0xc0f1b6a8, 0x31c399e2, 0x69772661
    ];
    assert_eq!((expected), (result));
}

#[test]
fn md5_compute_abc() {
    let msg = b"abc";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let result = md5(&parsed);
    let expected = [
        0x90015098, 0x3cd24fb0, 0xd6963f7d, 0x28e17f72
    ];
    assert_eq!((expected), (result));
}

#[test]
fn md5_compute_expenditure_next_second_block() {
    let msg = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcde\
                fghijklmnopqrstuvwxyz0123456789";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let result = md5(&parsed);
    let expected = [
        0xd174ab98, 0xd277d9f5, 0xa5611c2c, 0x9f419d9f
    ];
    assert_eq!((expected), (result));
}

#[test]
fn md5_compute_expenditure_into_next_block_with_all_numbers() {
    let msg = b"123456789012345678901234567\
                890123456789012345678901234\
                56789012345678901234567890";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let result = md5(&parsed);
    let expected = [
        0x57edf4a2, 0x2be3c955, 0xac49da2e, 0x2107b67a
    ];
    assert_eq!((expected), (result));
}