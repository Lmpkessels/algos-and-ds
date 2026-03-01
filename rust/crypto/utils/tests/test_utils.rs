use utils::utils::{
    z, shr, rotr, rotl, ch, maj, big_sigma0, big_sigma1, small_sigma0,
    small_sigma1
};

#[test]
fn computes_addition_modular_2_to_power_of_32_using_cast_down() {
    let x = 4294967295;
    let y = 4294967295;

    let result = z(x, y);
    let expected = 4294967294;

    assert_eq!((result), (expected));
}

#[test]
fn shift_x_right_by_n() {
    let x = 312;
    let n = 2;

    let result = shr(x, n);
    let expected = 78;

    assert_eq!((result), (expected));
}

#[test]
fn appends_bits_to_lsb_with_shifting_to_right() {
    let x = 15;
    let n = 3;

    let result = rotr(x, n);
    let expected = 3758096385;

    assert_eq!((result), (expected));
}

#[test]
fn appends_bits_to_msb_with_shifting_to_left() {
    let x = 3758096385;
    let n = 3;

    let result = rotl(x, n);
    let expected = 15;

    assert_eq!((result), (expected));
}

#[test]
fn ch_x_is_1_puts_out_y() {
    let x = 0xFFFFFFFF;
    let y = 0xAAAA_AAAA;
    let z = 0x5555_5555;

    let result = ch(x, y, z);
    let expected = y;

    assert_eq!((result), (expected));
}

#[test]
fn ch_x_is_0_puts_out_z() {
    let x = 0x000000000;
    let y = 0xAAAA_AAAA;
    let z = 0x5555_5555;

    let result = ch(x, y, z);
    let expected = z;

    assert_eq!((result), (expected));
}

#[test]
fn maj_computes_1_when_only_two_inputs_are_1() {
    let x = 0xFFFFFFFF;
    let y = 0xFFFFFFFF;
    let z = 0x00000000;
    
    let expected = 0xFFFFFFFF;
    let result = maj(x, y, z);

    assert_eq!((result), (expected));
}

#[test]
fn maj_computes_0_when_only_one_input_is_1() {
    let x = 0xFFFFFFFF;
    let y = 0x00000000;
    let z = 0x00000000;

    let result = maj(x, y, z);
    let expected = 0x00000000;

    assert_eq!(result, expected);
}

#[test]
fn big_sigma0_computes_expected_when_x_is_75() {
    let x = 75;

    let result = big_sigma0(x);
    let expected = rotr(75, 2) ^ rotr(75, 13) ^ rotr(75, 22);

    assert_eq!((result), (expected));
}

#[test]
fn big_sigma1_computes_expected_when_x_is_3() {
    let x = 3;

    let result = big_sigma1(x);
    let expected = rotr(3, 6) ^ rotr(3, 11) ^ rotr(3, 25);

    assert_eq!((result), (expected));
}

#[test]
fn small_sigma0_computes_expected_when_x_is_23() {
    let x = 23;
    let result = small_sigma0(x);

    let expected = rotr(23, 7) ^ rotr(23, 18) ^ shr(23, 3);

    assert_eq!((result), (expected));
}

#[test]
fn small_sigma1_computes_expected_when_x_is_64() {
    let x = 64;
    let result = small_sigma1(x);
    let expected = rotr(64, 17) ^ rotr(64, 19) ^ shr(64, 10);
    assert_eq!((result), (expected));
}