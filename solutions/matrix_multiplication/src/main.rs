use matrix_multiplication::{multiply, Matrix};

fn main() {
    let matrix = Matrix((1, 2), (3, 4));
    let val = 2;
    println!("Original matrix {:?}", matrix);
    println!("Matrix after multiply {:?}", multiply(matrix, val));
}

