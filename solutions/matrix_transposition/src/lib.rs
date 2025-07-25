#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(Matrix((a, b), (c, d)): Matrix) -> Matrix {
    Matrix((a, c), (b, d))
}