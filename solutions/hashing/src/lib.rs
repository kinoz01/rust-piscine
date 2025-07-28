use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        0.0
    } else {
        list.iter().map(|x| *x as f64).sum::<f64>() / list.len() as f64
    }
}

pub fn median(list: &[i32]) -> i32 {
    let mut v = list.to_vec();
    v.sort_unstable();
    let m = v.len() / 2;
    if v.len() % 2 == 1 { v[m] } else { (v[m - 1] + v[m]) / 2 }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut freq = HashMap::new();
    for &x in list { 
        *freq.entry(x).or_insert(0) += 1; 
    }
    freq.into_iter()
        .max_by_key(|&(_, c)| c)
        .map(|(val, _)| val)
        .unwrap_or(0)
}