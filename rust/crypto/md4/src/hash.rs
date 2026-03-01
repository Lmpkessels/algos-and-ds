use crate::functions::{ f, g, h };
use utils::utils::{ rotl, z };

/// MD4: Message Digestion function.
///
/// # Parameters
/// `m`: A vector of 512-bit message blocks, where each block is represented
///      as an array of sixteen 32-bit words in little-endian order.
///
/// # Description
/// - The algorithm performs three rounds of nonlinear functions, message word
///   reordering, bit rotations, and modular additions.  
/// - Each 512-bit message block updates the internal state `(A, B, C, D)`
///   to produce a 128-bit digest.
///
/// # Returns
/// A 128-bit digested hash key represented as an array of four 32-bit words.
///
/// # Reference
/// Based on the MD4 RFC-1320 specification:
/// [RFC-1320](https://datatracker.ietf.org/doc/html/rfc1320)
pub fn md4(m: &[[u32; 16]]) -> [u32; 4] {
    // Four word buffer for message digestion.
    // Order: A, B, C, D.
    let mut four_word_bffr: [u32; 4] = [
        0x67452301, // 0 = A
        0xefcdab89, // 1 = B
        0x98badcfe, // 2 = C
        0x10325476, // 3 = D
    ];

    // Index message order for 3 round.
    let message_order = [
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        ],
        [
            0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15
        ],
        [
            0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15
        ],
    ];
    
    // Rotl order for 3 rounds.
    let round_rotation = [
        [
            3, 7, 11, 19, 3, 7, 11, 19, 3, 7, 11, 19, 3, 7, 11, 19 
        ],
        [
            3, 5, 9, 13, 3, 5, 9, 13, 3, 5, 9, 13, 3, 5, 9, 13
        ],
        [
            3, 9, 11, 15, 3, 9, 11, 15, 3, 9, 11, 15, 3, 9, 11, 15
        ],    
    ];

    for x in m {
        // Buffer: (A, B, C, D) safed as lower: (a, b, c, d) instead of:
        // (AA, BB, CC, DD).
        let mut a = four_word_bffr[0];
        let mut b = four_word_bffr[1];
        let mut c = four_word_bffr[2];
        let mut d = four_word_bffr[3];

        for round in 0..3 {
            for j in 0..16 {
                let (func, msg, cnst, shft) = match round {
                    0 => (
                        // Round 1: nonlinear function F and no constant.
                        f(b, c, d), 
                        x[message_order[round][j]], 
                        0x00000000, 
                        round_rotation[round][j]),
                    1 => (
                        // Round 2: nonlinear function G with 0x5A827999.
                        g(b, c, d), x[message_order[round][j]], 
                        0x5A827999, 
                        round_rotation[round][j]),
                    2 => (
                        // Round 3: nonlinear function H with 0x6ED9EBA1.
                        h(b, c, d), 
                        x[message_order[round][j]], 
                        0x6ED9EBA1, 
                        round_rotation[round][j]),
                    _ => unreachable!()

                };

                let temp = a;
                a = d;
                d = c;
                c = b;
                b = rotl(z(z(z(temp, func), msg), cnst), shft);
            }

        }

        // Digest message.
        four_word_bffr[0] = z(four_word_bffr[0], a);
        four_word_bffr[1] = z(four_word_bffr[1], b);
        four_word_bffr[2] = z(four_word_bffr[2], c);
        four_word_bffr[3] = z(four_word_bffr[3], d);

    }

    // Digested state.
    [
        four_word_bffr[0].swap_bytes(), 
        four_word_bffr[1].swap_bytes(), 
        four_word_bffr[2].swap_bytes(), 
        four_word_bffr[3].swap_bytes()
    ]
}