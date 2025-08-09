pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|vstr| {
            let has_k = vstr.ends_with('k') || vstr.ends_with('K');
            let ns = if has_k { &vstr[..vstr.len() - 1] } else { vstr };
            let n = ns.parse::<f64>().unwrap();
            let v = if has_k { n * 1000.0 } else { n };
            Box::new(v as u32)
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|b| *b).collect()
}
