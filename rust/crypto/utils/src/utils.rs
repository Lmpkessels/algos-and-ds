/// Modular addition (x + y = (mod n))
///
/// Overflow is thrown away by the cast down to u32.
pub fn z(x: u32, y: u32) -> u32 {
    x.wrapping_add(y)
}

/// Logical right shift by n (pads with 0s at MSB).
pub fn shr(x: u32, n: u32) -> u32 {
    x >> n
}

pub fn parity(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }

/// Rotate right (ROTR).
/// 
/// Rotate x right by n bits within a 32-bit word (wraps bits around).
pub fn rotr(x: u32, n: u32) -> u32 {
    // Normalize 0..31.
    let n = n & 31;
    // Use the complement count within the 32-bit word.
    (x >> n) | (x << (32 - n))
}

/// Rotate left (ROTL).
///
/// Rotate x left by n bits within a 32-bit word (wraps bits around).
pub fn rotl(x: u32, n: u32) -> u32 {
    // Normalize 0..31.
    let n = n & 31;
    // Use the complement count within the 32-bit word.
    (x << n) | (x >> (32 - n))
}

/// Choose.
///
/// If 'x' is 1 then the output bit is 'y'.
/// If 'x' is 0 then the output bit is 'z'
pub fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!(x) & z)
}

/// Majority.
///
/// If at least 2 of the 3 inputs are 1, output is 1.
/// Otherwise output is 0.
pub fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

/// Big sigma 0 (Upper case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 2 bits.
/// - Rotate 'x' right by 13 bits.
/// - Rotate 'x' right by 22 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn big_sigma0(x: u32) -> u32 {
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

/// Big sigma 1 (Upper case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 6 bits.
/// - Rotate 'x' right by 11 bits.
/// - Rotate 'x' right by 25 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn big_sigma1(x: u32) -> u32 {
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

/// Small sigma 0 (Lower case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 7 bits.
/// - Rotate 'x' right by 18 bits.
/// - Shift 'x' right by 3 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn small_sigma0(x: u32) -> u32 {
    rotr(x, 7) ^ rotr(x, 18) ^ shr(x, 3)
}

/// Small sigma 1 (Lower case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 17 bits.
/// - Rotate 'x' right by 19 bits.
/// - Shift 'x' right by 10 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn small_sigma1(x: u32) -> u32 {
    rotr(x, 17) ^ rotr(x, 19) ^ shr(x, 10)
}