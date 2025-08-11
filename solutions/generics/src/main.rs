// Write a functions called identity that calculates the identity of a
// value (receives any data type and returns the same value)
use generics::*;

fn main() {
    println!("{}", identity("Hello, world!"));
    println!("{}", identity(3));
}
