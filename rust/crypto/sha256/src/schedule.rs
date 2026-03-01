use utils::utils::{
    z, small_sigma1, small_sigma0
};

/// SHA256 Schedule function.
///
/// # Arguments:
/// Takes blocks as Vec<[u32; 16]>.
/// Then takes each 512-bit block outside of the blocks vector.
///
/// # Description
/// - Copies each block into m[0..16].
/// - Expands words 16..63 using small_sigma0 and small_sigma1 per SHA-256 spec.
/// - Produces a full 64-word schedule for each block.
///
/// # Returns
/// Scheduled message as vector [u32; 64] for downstream compression.
pub fn schedule(blocks: Vec<[u32; 16]>) -> Vec<[u32; 64]> {
    let mut schedule: Vec<[u32; 64]> = Vec::new();

    for block in blocks {
        let mut m = [0u32; 64];
        
        for t in 0..16 {
            m[t] = block[t];
        }
        
        for t in 16..64 {
            m[t] = z(
                z(small_sigma1(m[t-2]), m[t-7]), 
                z(small_sigma0(m[t-15]), m[t-16])
            );
        }
        schedule.push(m);
    }
    schedule
}