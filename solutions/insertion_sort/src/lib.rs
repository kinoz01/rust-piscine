pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    let n = slice.len();

    for i in 1..=steps.min(n - 1) {
        let key = slice[i];
        let mut j = i;

        while j > 0 && slice[j - 1] > key {
            slice[j] = slice[j - 1];
            j -= 1;
        }
        slice[j] = key;
    }
}