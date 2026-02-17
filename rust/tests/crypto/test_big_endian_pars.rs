use rust::crypto::padd_pars::big_endian_padd;
use rust::crypto::padd_pars::big_endian_pars;

#[test] 
fn pads_message_to_16_32_bit_words() {
    let bytes = big_endian_padd(b"abc"); 
    let result = big_endian_pars(bytes); 
    let expected_block: [u32; 16] = [
        0x61626380, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000018
    ];
    let expected: Vec<[u32; 16]> = vec![expected_block];
    assert_eq!((result), (expected));
}
#[test]
fn pads_message_to_16_32_bit_words_and_expands_into_second_block() {
    let bytes = big_endian_padd(b"AAAAA_AAAAA_AAAAA_AAAAA_AAAAA_AAAAA_\
    AAAAA_AAAAA_AAAAA_AAAAA_AAAAA");
    let result = big_endian_pars(bytes);
    // First 512-bit block
    let first_block: [u32; 16] = [
        1094795585, 1096761665, 1094795615, 1094795585, 
        1096761665, 1094795615, 1094795585, 1096761665, 
        1094795615, 1094795585, 1096761665, 1094795615, 
        1094795585, 1096761665, 1094795615, 1094795585
    ];
    // Second 512-bit block (from padding + length)
    let second_block: [u32; 16] = [
        1098907648, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 520
    ];
    let expected: Vec<[u32; 16]> = vec![first_block, second_block];
    assert_eq!(result, expected);
}
#[test] 
fn small_endian_pars_test() {
    let bytes = big_endian_padd(b"abc"); 
    let result = big_endian_pars(bytes); 
    let expected_block: [u32; 16] = [
        0x61626380, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000018
    ];
    let expected: Vec<[u32; 16]> = vec![expected_block];
    assert_eq!((result), (expected));
}
