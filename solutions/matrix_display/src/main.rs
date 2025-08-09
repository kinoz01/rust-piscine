// Implement the std::fmt::Display trait for a matrix of i32
// using the struct Matrix define the associated function new that
// creates a new Matrix from &[&[i32]]
// After implement the std::fmt::Display trait to print the matrix
// like this

// ```
// (1 2 3)
// (4 5 6)
// (7 8 9)
// ```

use matrix_display::*;

fn main() {
    let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    println!("{}", matrix);
}

