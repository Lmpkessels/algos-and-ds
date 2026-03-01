/// Boolean functions for RIMEMD-160.
///
/// All 5 functions apply bitwise logic bit-by-bit on 32 bit words.
/// All 5 functions will be used during the hash rounds computation.
///
/// #Reference
/// All round boolean functions (1-5) are gained from:
/// [KULeuven Report AB-9601](https://homes.esat.kuleuven.be/~bosselae/ripemd160/pdf/AB-9601/AB-9601.pdf)
pub fn f1(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }

pub fn f2(x: u32, y: u32, z: u32) -> u32 { (x & y) | (!(x) & z) }

pub fn f3(x: u32, y: u32, z: u32) -> u32 { (x | !(y)) ^ z }

pub fn f4(x: u32, y: u32, z: u32) -> u32 { (x & z) | (y & !(z)) }

pub fn f5(x: u32, y: u32, z: u32) -> u32 { x ^ (y | !(z)) }

/// The five left round functions.
///
/// Use the range of j to decide which function to take from 0..79.
pub fn f_left(j: u32, x: u32, y: u32, z: u32) -> u32 {
    match j {
        0..=15 => f1(x, y, z),
        16..=31 => f2(x, y, z),
        32..=47 => f3(x, y, z),
        48..=63 => f4(x, y, z),
        64..=79 => f5(x, y, z),
        _ => unreachable!(),
    }
}

/// The five right round functions.
///
/// Use the range of j to decide which function to take from 0..79.
/// Here the function order is reversed for the second round (79..0).
pub fn f_right(j: u32, x: u32, y: u32, z: u32) -> u32 {
    match j {
        0..=15 => f5(x, y, z),
        16..=31 => f4(x, y, z),
        32..=47 => f3(x, y, z),
        48..=63 => f2(x, y, z),
        64..=79 => f1(x, y, z),
        _ => unreachable!(),
    }
}