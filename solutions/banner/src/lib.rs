use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag<'a> {
    pub short_hand: String,
    pub long_hand: String,
    pub desc:       &'a str,
}

impl<'a> Flag<'a> {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Self {
            short_hand: format!("-{}", name.chars().next().unwrap()),
            long_hand:  format!("--{name}"),
            desc:       d,
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag<'a>(&mut self, flag: Flag<'a>, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand,  func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let cb = self.flags.get(input).ok_or("flag not found")?;
        cb(argv[0], argv[1]).map_err(|e| e.to_string())
    }
}

// callback functions
pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.parse()?;
    Ok((x / y).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.parse()?;
    Ok((x % y).to_string())
}
