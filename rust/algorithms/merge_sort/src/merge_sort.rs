fn merge_sort(main_vector: Vec<u32>) -> Vec<u32> {
    // Don't sort if length <= 1.
    if main_vector.len() <= 1 {
        return main_vector;
    };

    // Get the middle of the entire length.
    let middle = main_vector.len() / 2;

    let mut left_vector: Vec<u32> = Vec::new();
    let mut right_vector: Vec<u32> = Vec::new();

    // Fill left subsequence.
    for i in 0..middle {
        left_vector.push(main_vector[i]);
    }

    // Fill right subsequence.
    for j in middle..main_vector.len() {
        right_vector.push(main_vector[j]);
    }

    // Order both sides.
    let left_side_sorted = merge_sort(left_vector);
    let right_side_sorted = merge_sort(right_vector);
    
    let mut merged_sorted = Vec::with_capacity(main_vector.len());
    let mut k = 0;
    let mut l = 0;

    // Check both sides on equality.
    while k < left_side_sorted.len() && l < right_side_sorted.len() {
        if left_side_sorted[k] < right_side_sorted[l] {
            merged_sorted.push(left_side_sorted[k]);
            k += 1;
        } else {
            merged_sorted.push(right_side_sorted[l]);
            l += 1;
        }
    }

    // Push remaining left subsequence indises.
    while k < left_side_sorted.len() {
        merged_sorted.push(left_side_sorted[k]);
        k += 1;
    }

    // Push remaining right subsequence indises.
    while l < right_side_sorted.len() {
        merged_sorted.push(right_side_sorted[l]);
        l += 1;
    }

    merged_sorted
}

fn main() {
    let test = merge_sort(vec![0, 11, 2332, 1, 9999, 13, 11]);
    println!("{test:?}");
}
