pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().rposition(|&x| x == key)
}