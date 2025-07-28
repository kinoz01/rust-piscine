use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map = HashMap::with_capacity(words.len());
    for &w in words {
        *map.entry(w).or_insert(0) += 1;
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}