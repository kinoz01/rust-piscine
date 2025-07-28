pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut total: u64 = arr.iter().sum();
    let mut res = Vec::with_capacity(arr.len()+1);
    res.push(total);

    for &x in arr.iter().rev() {
        total -= x;
        res.push(total);
    }
    res
}