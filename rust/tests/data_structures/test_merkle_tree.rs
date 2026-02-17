use rust::crypto::sha256::sha256;
use rust::data_structures::merkle_tree::{ 
    branching, leaf_loading, merkle_tree
};

#[test]
fn load_even_leaf_then_branch_data_and_compute_root() {
    let a = [0x5cu8; 32];
    let b = [0x36u8; 32];
    let c = [0xcdu8; 32];
    let d = [0xa3u8; 32];
    let e = [0x94u8; 32];
    let f = [0x30u8; 32];
    let g = [0x08u8; 32];
    let h = [0x23u8; 32];
    let ha = sha256(&a);
    let hb = sha256(&b);
    let hc = sha256(&c);
    let hd = sha256(&d);
    let he = sha256(&e);
    let hf = sha256(&f);
    let hg = sha256(&g);
    let hh = sha256(&h);
    let hab = sha256(&[ha, hb].concat());
    let hcd = sha256(&[hc, hd].concat());
    let hef = sha256(&[he, hf].concat());
    let hgh = sha256(&[hg, hh].concat());
    let habcd = sha256(&[hab, hcd].concat());
    let hefgh = sha256(&[hef, hgh].concat());
    let result = merkle_tree(vec![a, b, c, d, e, f, g, h]);
    let expected = sha256(&[habcd, hefgh].concat());
    assert_eq!((result), (expected));
}

#[test]
fn load_odd_leaf_then_branch_data_and_compute_root() {
    let a = [0x94u8; 32];
    let b = [0x08u8; 32];
    let c = [0x30u8; 32];
    let d = [0x5cu8; 32];
    let e = [0x9fu8; 32];
    let f = [0xffu8; 32];
    let g = [0x23u8; 32];
    
    let ha = sha256(&a);
    let hb = sha256(&b);
    let hc = sha256(&c);
    let hd = sha256(&d);
    let he = sha256(&e);
    let hf = sha256(&f);
    let hg = sha256(&g);
    let hab = sha256(&[ha, hb].concat());
    let hcd = sha256(&[hc, hd].concat());
    let hef = sha256(&[he, hf].concat());
    let hgg = sha256(&[hg, hg].concat());
    let habcd = sha256(&[hab, hcd].concat());
    let hefgh = sha256(&[hef, hgg].concat());
    let result = merkle_tree(vec![a, b, c, d, e, f, g]);
    let expected = sha256(&[habcd, hefgh].concat());
    assert_eq!((result), (expected));
}

#[test] 
fn load_and_hash_leaf_then_store_them() {
    let a = [0x19u8; 32];
    let b = [0xf2u8; 32];
    let ha = sha256(&a);
    let hb = sha256(&b);
    let result = leaf_loading(&vec![a, b]);
    let expected = vec![ha, hb];
    assert_eq!((result), (expected))
}

#[test]
fn load_leafs_increment_index_3_and_hash_all_values() {
    let a = [0x64u8; 32];
    let b = [0x3cu8; 32];
    let c = [0x6fu8; 32];
    
    let ha = sha256(&a);
    let hb = sha256(&b);
    let hc = sha256(&c);
    
    let result = leaf_loading(&vec![a, b, c]);
    let expected = vec![ha, hb, hc, hc];
    assert_eq!((result), (expected));
}

#[test]
fn append_hashed_leafs_1by1_and_hash_appended_computation() {
    let a = [0xccu8; 32];
    let b = [0x1du8; 32];
    
    let mut appended_leafs = Vec::with_capacity(64);
    for i in 0..a.len() {
        appended_leafs.push(a[i]);
    }
    for j in 0..b.len() {
        appended_leafs.push(b[j]);
    }
    let result = branching(a, b);
    let expected = sha256(&appended_leafs);
    assert_eq!((result), (expected));
}
