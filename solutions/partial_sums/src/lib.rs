pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut res = Vec::with_capacity(arr.len()+1);
    let mut tot = arr.iter().sum();
    res.push(tot);

    for n in arr.iter().rev() {
        tot -= n;
        res.push(tot);
    }
    res
}