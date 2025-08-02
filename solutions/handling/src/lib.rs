use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                panic!("{e}");
            }
        }
        Err(e) => panic!("{e}"),
    }
}