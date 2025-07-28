pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut v1: Vec<char> = s1.chars().collect();
    let mut v2: Vec<char> = s2.chars().collect();

    v1.sort_unstable();
    v2.sort_unstable();

    v1 == v2
}