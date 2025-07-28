use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    *h.values().min().unwrap_or(&i32::MAX)
}