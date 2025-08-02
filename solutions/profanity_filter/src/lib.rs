pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}