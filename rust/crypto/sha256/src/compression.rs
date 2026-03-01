use utils::utils::{ 
   z, big_sigma1, big_sigma0, ch, maj
};

/// SHA256 compression function for message digestion.
///
/// # Argument
/// Takes prepared message schedule as Vec<[u32; 64]>. 
/// Then takes each scheduled block and compression starts.
///
/// # Description
/// - Initialize (a, b, c, d, e, f, g, h), with the eight working variables; 
///   (h0, h1, h2, h3, h4, h5, h6, h7), with the (m) hash value.
/// - Implement all round operations (Ch, Maj, Big_sigma1, Big_sigma_0).
/// - Compute the i-th intermediate hash value H(i)
///
/// # Returns
/// Final 8-word digest as [u32; 8].
pub fn compress(schedule: Vec<[u32; 64]>) -> [u32; 8] {
    // Hash values.
    let mut h0: u32 = 0x6a09e667;
    let mut h1: u32 = 0xbb67ae85;
    let mut h2: u32 = 0x3c6ef372;
    let mut h3: u32 = 0xa54ff53a;
    let mut h4: u32 = 0x510e527f;
    let mut h5: u32 = 0x9b05688c;
    let mut h6: u32 = 0x1f83d9ab;
    let mut h7: u32 = 0x5be0cd19;

    // Round constants.
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    for m in schedule {
        // Initialize working variables.
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        let mut f = h5;
        let mut g = h6;
        let mut h = h7;
        
        // Implement round operations.
        for i in 0..64 {
            let t1 = z(z(z(z(h, big_sigma1(e)), ch(e, f, g)), K[i]), m[i]);
            let t2 = z(big_sigma0(a), maj(a, b, c));
            h = g;
            g = f;
            f = e;
            e = z(d, t1);
            d = c;
            c = b;
            b = a;
            a = z(t1, t2);
        }
        
        // Compute the i-th intermediate hash value H(i)
        h0 = z(h0, a);
        h1 = z(h1, b);
        h2 = z(h2, c);
        h3 = z(h3, d);
        h4 = z(h4, e);
        h5 = z(h5, f);
        h6 = z(h6, g);
        h7 = z(h7, h);
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