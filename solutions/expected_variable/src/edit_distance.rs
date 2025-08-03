pub fn edit_distance(source: &str, target: &str) -> usize {
    let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();

    let mut prev: Vec<usize> = (0..=t.len()).collect(); 

    for (i, &sc) in s.iter().enumerate() {
        let mut curr = vec![i + 1];
        for (j, &tc) in t.iter().enumerate() {
            let cost = if sc == tc { 0 } else { 1 };
            let dist = (prev[j + 1] + 1)
                .min(curr[j] + 1)
                .min(prev[j] + cost);
                curr.push(dist);
        }
        prev = curr;
    }
    
    prev[t.len()]
}
