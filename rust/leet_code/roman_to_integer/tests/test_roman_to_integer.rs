use roman_to_integer::roman_to_integer::roman_to_int;

#[test]
fn test_roman_to_int_with_iv_and_ix_to_get_4_and_9() {
    let iv = "IV".to_string();
    let ix = "IX".to_string();

    const IV_EXPECTED: i32 = 4;
    const IX_EXPECTED: i32 = 9;
    
    assert_eq!((IV_EXPECTED), (roman_to_int(iv)));
    assert_eq!((IX_EXPECTED), (roman_to_int(ix)));
}

#[test]
fn test_roman_to_int_with_xl_and_xc_to_get_40_and_90() {
    let xl = "XL".to_string();
    let xc = "XC".to_string();

    const XL_EXPECTED: i32 = 40;
    const XC_EXPECTED: i32 = 90;

    assert_eq!((XL_EXPECTED), (roman_to_int(xl)));
    assert_eq!((XC_EXPECTED), (roman_to_int(xc)));
}

#[test]
fn test_roman_to_int_with_cd_and_cm_to_get_400_and_900() {
    let cd = "CD".to_string();
    let cm = "CM".to_string();

    const CD_EXPECTED: i32 = 400;
    const CM_EXPECTED: i32 = 900;

    assert_eq!((CD_EXPECTED), (roman_to_int(cd)));
    assert_eq!((CM_EXPECTED), (roman_to_int(cm)));
}

#[test]
fn test_roman_to_int_for_iii_lviii_mcmxciv() {
    let iii = "III".to_string();
    let lviii = "LVIII".to_string();
    let mcmxciv = "MCMXCIV".to_string();

    const III_EXPECTED: i32 = 3;
    const LVIII_EXPECTED: i32 = 58;
    const MCMXCIV_EXPECTED: i32 = 1994;

    assert_eq!((III_EXPECTED), (roman_to_int(iii)));
    assert_eq!((LVIII_EXPECTED), (roman_to_int(lviii)));
    assert_eq!((MCMXCIV_EXPECTED), (roman_to_int(mcmxciv)));
}