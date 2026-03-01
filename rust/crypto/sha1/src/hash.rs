use utils::utils::{ rotl, z };
use crate::functions::f;
use crate::constants::k;

/// SHA1 Message Digestion Algorithm.
///
/// # Argument
/// Takes scheduled message as Vec<[u32; 80]>, then compression starts.
///
/// # Description
/// - Initialize (a, b, c, d, e), with the 5 working variables; 
///   (h0, h1, h2, h3, h4).
/// - Implement all round operations (Ch, Maj, Parity, ROTL).
/// - Compute the i-th intermediate hash value H(i)
///
/// # Returns
/// Final 8-word digest as [u32; 5].
///
/// # Reference
/// Based on the FIPS PUB 180-4 specification:
/// [FIPS PUB 180-4](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.180-4.pdf)
pub fn sha1(msg: &[[u32; 80]]) -> [u32; 5] {
    // Initialize working variables.
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xefcdab89;
    let mut h2: u32 = 0x98badcfe;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xc3d2e1f0;

    for m in msg {
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        // Implement round operations.
        for t in 0..80 {

            let temp = z(
                z(
                    z(
                        z(
                            rotl(a, 5), 
                            f(t, b, c, d)), 
                        e), 
                    k(t)), 
                m[t as usize]
            );

            e = d;
            d = c;
            c = rotl(b, 30);
            b = a;
            a = temp;

        };

        // Compute the i-th intermediate hash value.
        h0 = z(a, h0);
        h1 = z(b, h1);
        h2 = z(c, h2);
        h3 = z(d, h3);
        h4 = z(e, h4);

    }

    // Digested state.
    [
        h0, 
        h1, 
        h2, 
        h3, 
        h4
    ]

}