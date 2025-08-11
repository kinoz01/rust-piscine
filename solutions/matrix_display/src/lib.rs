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
        for (ri, row) in self.0.iter().enumerate() {
            write!(f, "(")?;
            for (ci, n) in row.iter().enumerate() {
                if ci > 0 {write!(f, " ")?;}
                write!(f, "{}", n)?;
            }
            write!(f, ")")?;
            if ri+1 < self.0.len() {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}