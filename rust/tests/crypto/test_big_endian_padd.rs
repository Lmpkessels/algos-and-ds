use rust::crypto::padd_pars::big_endian_padd;

#[test]
fn takes_msg_appends_1_k_and_msg_length_computes_to_vec_of_bytes() {
    let msg = b"abc";
    let result = big_endian_padd(msg);
    let expected = [
        97, 98, 99, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 24
    ];
    
    assert_eq!((result), (expected));
}
