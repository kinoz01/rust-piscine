pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    use Security::*;
    match security_level {
        Unknown => server.unwrap().to_string(),
        Message => server.expect("ERROR: program stops").to_string(),
        Warning => server.unwrap_or("WARNING: check the server").to_string(),
        NotFound => server.map_or_else(|e| format!("Not found: {}", e), String::from),
        UnexpectedUrl => server.unwrap_err().to_string(),
    }
}