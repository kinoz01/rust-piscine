pub fn scytale_cipher(message: &str, size: usize) -> String {
    if size == 0 {
        return String::new();
    }

    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + size - 1) / size;

    let mut result = String::with_capacity(rows * size);
    for col in 0..size {
        for row in 0..rows {
            let idx = row * size + col;
            if idx < len {
                result.push(chars[idx]);
            } else {
                result.push(' ');
            }
        }
    }
    result.trim_end().to_string()
}