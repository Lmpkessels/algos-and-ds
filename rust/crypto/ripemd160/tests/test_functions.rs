use ripemd160::functions::{
    f1, f2, f3, f4, f5, f_left, f_right
};

#[test]
fn compute_f1_return_x_xor_y_xor_z() {
    let x = 32;
    let y = 18;
    let z = 122;
    
    let result = f1(x, y, z);
    let expected = x ^ y ^ z;

    assert_eq!((result), (expected));
}

#[test]
fn compute_f2_return_x_and_y_or_not_x_and_z() {
    let x = 68;
    let y = 99;
    let z = 128;

    let result = f2(x, y, z);
    let expected = (x & y) | (!(x) & z);

    assert_eq!((result), (expected));
}

#[test]
fn compute_f3_return_x_or_not_y_xor_z() {
    let x = 234;
    let y = 123;
    let z = 321;

    let result = f3(x, y, z);
    let expected = (x | !(y)) ^ z;

    assert_eq!((result), (expected));
}

#[test]
fn compute_f4_return_x_and_z_or_y_and_not_z() {
    let x = 128;
    let y = 45;
    let z = 512;
    
    let result = f4(x, y, z);
    let expected = (x & z) | (y & !(z));

    assert_eq!((result), (expected));
}

#[test]
fn compute_f5_return_x_xor_y_or_not_z() {
    let x = 9;
    let y = 30;
    let z = 8;

    let result = f5(x, y, z);
    let expected = x ^ (y | !(z));

    assert_eq!((result), (expected));
}

#[test]
fn test_f() {
    let expected_fn_1 = f1(23, 11, 14);
    let expected_fn_2 = f2(23, 11, 14);
    let expected_fn_3 = f3(23, 11, 14);
    let expected_fn_4 = f4(23, 11, 14);
    let expected_fn_5 = f5(23, 11, 14);        
    
    let result_zero = f_left(0, 23, 11, 14);
    let result_one = f_left(16, 23, 11, 14);
    let result_two = f_left(32, 23, 11, 14);
    let result_three = f_left(48, 23, 11, 14);
    let result_four = f_left(64, 23, 11, 14);

    assert_eq!((expected_fn_1), (result_zero));
    assert_eq!((expected_fn_2), (result_one));
    assert_eq!((expected_fn_3), (result_two));
    assert_eq!((expected_fn_4), (result_three));
    assert_eq!((expected_fn_5), (result_four));
}

#[test]
fn test_f_reversed() {
    let expected_fn_5 = f5(23, 11, 14);        
    let expected_fn_4 = f4(23, 11, 14);
    let expected_fn_3 = f3(23, 11, 14);
    let expected_fn_2 = f2(23, 11, 14);
    let expected_fn_1 = f1(23, 11, 14);
    
    let result_zero = f_right(0, 23, 11, 14);
    let result_one = f_right(16, 23, 11, 14);
    let result_two = f_right(32, 23, 11, 14);
    let result_three = f_right(48, 23, 11, 14);
    let result_four = f_right(64, 23, 11, 14);

    assert_eq!((expected_fn_5), (result_zero));
    assert_eq!((expected_fn_4), (result_one));
    assert_eq!((expected_fn_3), (result_two));
    assert_eq!((expected_fn_2), (result_three));
    assert_eq!((expected_fn_1), (result_four));
}