use crate::crypto::sha256::sha256;

/// Merkle-tree branching: combine left and right nodes into a parent node.
///
/// # Arguments
/// - `left_node` - 32-byte array representing the left child.
/// - `right_node` - 32-byte array representing the right child.
///
/// # Description
/// - Append all bytes from the left node into a 64-byte vector.
/// - Append all bytes from the right node into the same vector.
/// - Hash the combined 64 bytes to create the parent node.
///
/// # Returns
/// - `[u8; 32]` - the parent node hash.
pub fn branching(left_node: [u8; 32], right_node: [u8; 32]) -> [u8; 32] {
    let mut combined = Vec::with_capacity(64);

    for i in 0..left_node.len() {
        combined.push(left_node[i]);
    }

    for j in 0..right_node.len() {
        combined.push(right_node[j]);
    }

    sha256(&combined)
}

/// Merkle-tree leaf loading: to increment if needed, and hash transaction.
///
/// # Arguments
/// - `leaf` - A vector of 32-byte arrays, each representing a transaction.
///
/// # Description
/// - If the number of leaf nodes is odd, duplicate the last node.
/// - Hash each transaction to create the leaf nodes.
///
/// # Returns
/// Hashed version of leaf nodes (hashed transactions).
pub fn leaf_loading(leaf: &[[u8; 32]]) -> Vec<[u8; 32]> {
    let mut transactions = leaf.to_vec();
    
    if transactions.len() % 2 != 0 {
        // Duplicate the last element if odd.
        let last_index = transactions[transactions.len() - 1];
        transactions.push(last_index);
    }; 

    let mut leaf_nodes: Vec<[u8; 32]> = Vec::new();
    for i in &transactions {
        // Hash and store data.
        let hashed_transaction = sha256(i);    
        leaf_nodes.push(hashed_transaction);
    }

    leaf_nodes
}

/// Merkle tree: for efficiently verifying data.
///
/// # Arguments
/// - `leaf` - A vector of 32-byte arrays, each representing a transaction.
///
/// # Description
/// - Hash each transaction to create the leaf nodes.
/// - If the number of leaf nodes is odd, duplicate the last node.
/// - While more than one node remains:
///   - Pair nodes by index (0-1, 2-3, 4-5, etc.).
///   - Concatenate left (even index) and right (odd index).
///   - Hash the combined pair to create the parent node.
/// - Repeat until only one node remains, the -> Merkle root.
///
/// # Returns
/// - `[u8; 32]` - the Merkle root of the tree.
///
/// # References
/// - [Investopedia](https://www.investopedia.com/terms/m/merkle-tree.asp)  
/// - [Bitcoin developer guide](https://developer.bitcoin.org/devguide/block_chain.html)
pub fn merkle_tree(leaf: Vec<[u8; 32]>) -> [u8; 32] {
    let mut leaf_nodes = leaf_loading(&leaf);

    while leaf_nodes.len() > 1 {
        let mut non_leaf_nodes: Vec<[u8; 32]> = Vec::new();
        if leaf_nodes.len() % 2 != 0 {
            // Increment if odd.
            let last_index = leaf_nodes[leaf_nodes.len() - 1];
            leaf_nodes.push(last_index);
        };

        let mut i = 0;
        while i < leaf_nodes.len() {
            let parent_node = branching(leaf_nodes[i], leaf_nodes[i + 1]);
            non_leaf_nodes.push(parent_node);
            // Get index per 2 (0-1, 2-4, 5-6, etc).
            i = i + 2;
        }

        leaf_nodes = non_leaf_nodes;
    }

    let merkle_root = leaf_nodes[0];
    merkle_root
}