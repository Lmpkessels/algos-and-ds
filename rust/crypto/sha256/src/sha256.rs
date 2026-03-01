use crate::sha256::{
    big_endian_padd, big_endian_pars
};

use crate::sha256::{
    schedule, compress, to_bytes
};

/// SHA-256: Pads, parses, schedules, and compresses a message into a 256-bit 
/// hash.
///
/// # Arguments
/// - `msg`: Message as a byte slice (`&[u8]`). 
///
/// # Description
/// - **Padding:** Appends a single '1' bit, then `k` zero bits so that the
///   total length ≡ 448 (mod 512). Finally appends the original message
///   length as a 64-bit big-endian integer.
/// - **Parsing:** Splits the padded message into 512-bit blocks (16 × 32-bit words).
/// - **Scheduling:** Expands each block into 64 words using the `σ0` and `σ1` functions.
/// - **Compression:** Iteratively updates the hash state across all blocks
///   to compute the final digest.
///
/// # Returns
/// A 32-byte array representing the 256-bit hash.
///
/// # Notes
/// SHA-256 supports input messages up to `2^64 − 1` bits (≈ 2.3 exabytes).
/// 
/// # Reference
/// [FIPS PUB 180-4](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.180-4.pdf)
pub fn sha256(msg: &[u8]) -> [u8; 32] {
    let padding = big_endian_padd(msg);
    let parsing = big_endian_pars(padding);
    let scheduled = schedule(parsing);
    let digest = compress(scheduled);
    let bytes = to_bytes(digest);

    bytes
}