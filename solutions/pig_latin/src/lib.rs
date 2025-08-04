pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    text
        .split_whitespace()
        .map(|word| {
            let first = word.chars().next().unwrap_or_default();

            if vowels.contains(&first) {
                format!("{}ay", word)

            } else if word.len() >= 3
                && !vowels.contains(&first)
                && &word[1..3] == "qu"
            {
                let (head, tail) = word.split_at(3);
                format!("{}{}ay", tail, head)

            } else if let Some(idx) = word.find(|c: char| vowels.contains(&c)) {
                let (head, tail) = word.split_at(idx);
                format!("{}{}ay", tail, head)

            } else {
                format!("{}ay", word)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}