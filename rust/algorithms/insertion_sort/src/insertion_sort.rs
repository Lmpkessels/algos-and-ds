fn insertion_sort(mut a: Vec<u32>) -> Vec<u32> {
    // Current index.
    for i in 0..a.len() {
        // One index ahead.
        for j in i + 1..a.len() {
            // Check greater then and sort less then first.
            if a[i] > a[j] {
                let temp = a[i];
                a[i] = a[j];
                a[j] = temp;
            }
        }
    }

    a
}

fn main() {
    let test = insertion_sort(vec![111, 21, 121, 1, 11, 12, 99]);
    println!("{test:?}");
}
