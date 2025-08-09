#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        Matrix(slice.iter().map(|row| row.to_vec()).collect())
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lines: Vec<String> = self.0.iter()
            .map(|row| {
                let body = row.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("({})", body)
            })
            .collect();
        write!(f, "{}", lines.join("\n"))
    }
}
