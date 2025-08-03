#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boss {
    pub name: String,
    pub age:  u32,
}

impl Boss {
    pub fn new(name: &str, age: u32) -> Self {
        Self { name: name.to_string(), age }
    }
}
