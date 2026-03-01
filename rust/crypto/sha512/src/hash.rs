use utils::utils64::{
    big_sigma0_64, big_sigma1_64, ch64, maj64, z64
};

use crate::constants::K;

/// SHA512 Message Digestion Algorithm.
///
/// # Argument
/// Takes scheduled message as Vec<[u64; 80]>, then compression starts.
///
/// # Description
/// - Initialize (a, b, c, d, e, f, g, h), with the 8 working variables: 
///   (h0, h1, h2, h3, h4, h5, h6, h7).
/// - Implement all round operations (Big sigma 0 & 1, Small sigma 0 & 1,
///   Ch, Maj).
/// - Compute the i-th intermediate hash value H(i)
///
/// # Returns
/// Final 8-word digest as [u64; 8] (512 bits).
///
/// # Reference
/// Based on the FIPS PUB 180-4 specification:
/// [FIPS PUB 180-4](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.180-4.pdf)
pub fn sha512(msg_blocks: &[[u64; 80]]) -> [u64; 8] {
    // Hash values.
    let mut h0: u64 = 0x6a09e667f3bcc908;
    let mut h1: u64 = 0xbb67ae8584caa73b;
    let mut h2: u64 = 0x3c6ef372fe94f82b;
    let mut h3: u64 = 0xa54ff53a5f1d36f1;
    let mut h4: u64 = 0x510e527fade682d1;
    let mut h5: u64 = 0x9b05688c2b3e6c1f;
    let mut h6: u64 = 0x1f83d9abfb41bd6b;
    let mut h7: u64 = 0x5be0cd19137e2179;

    for w in msg_blocks {
        // Initialized working variables.
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        let mut f = h5;
        let mut g = h6;
        let mut h = h7;

        // Implement 80 round operations.
        for t in 0..80 {

            let temp1 = z64(
                z64(
                    z64(
                        z64(
                            h, 
                            big_sigma1_64(e)), 
                        ch64(e, f, g)), 
                    K[t]), 
                w[t]
            );
            let temp2 = z64(
                big_sigma0_64(a), 
                maj64(a, b, c)
            );
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);

        }

        // Compute the i-th intermediate hash value H(i).
        h0 = z64(a, h0);
        h1 = z64(b, h1);
        h2 = z64(c, h2);
        h3 = z64(d, h3);
        h4 = z64(e, h4);
        h5 = z64(f, h5);
        h6 = z64(g, h6);
        h7 = z64(h, h7);

    }

    // Digested state.
    [
        h0,
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        h7
    ]
}