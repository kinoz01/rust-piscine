pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    assert!(v.is_char_boundary(index), "Invalid UTF-8 boundary at index {}", index);
    v.split_at(index)
    // (&v[..index], &v[index..]) can also be used
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("character not found")
    // unwrap() or unwrap_or(default) can also be used
}