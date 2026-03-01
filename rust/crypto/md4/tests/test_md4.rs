use padd_pars::{little_endian_padd, little_endian_pars};
use md4::hash::md4;

#[test]
fn md4_compute_empty_string() {
    let msg = b"";
    let padded = little_endian_padd(msg); 
    let parsed = little_endian_pars(padded);
    let result = md4(&parsed);
    let expected = [
        0x31d6cfe0, 0xd16ae931, 0xb73c59d7, 0xe0c089c0
    ];
    assert_eq!((result), (expected));
}

#[test]
fn md4_compute_a() {
    let msg = b"a";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let result = md4(&parsed);
    let expected = [
        0xbde52cb3, 0x1de33e46, 0x245e05fb, 0xdbd6fb24
    ];
    assert_eq!((result), (expected));
}

#[test]
fn md4_compute_abc() {
    let msg = b"abc";
    let padded = little_endian_padd(msg); 
    let parsed = little_endian_pars(padded);
    let result = md4(&parsed);
    let expected = [
        0xa448017a, 0xaf21d852, 0x5fc10ae8, 0x7aa6729d
    ];
    assert_eq!((result), (expected));
}

#[test]
fn md4_compute_long_abcde_string() {
    let msg = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcde\
                fghijklmnopqrstuvwxyz0123456789";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let result = md4(&parsed);
    let expected = [
        0x043f8582, 0xf241db35, 0x1ce627e1, 0x53e7f0e4
    ];
    assert_eq!((result), (expected));
}

#[test]
fn md4_compute_long_numbers_string() {
    let msg = b"1234567890123456789012345678901234567890\
                1234567890123456789012345678901234567890";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let result = md4(&parsed);
    let expected = [
        0xe33b4ddc, 0x9c38f219, 0x9c3e7b16, 0x4fcc0536
    ];
    assert_eq!((result), (expected));
}