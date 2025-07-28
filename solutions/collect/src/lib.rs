pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}