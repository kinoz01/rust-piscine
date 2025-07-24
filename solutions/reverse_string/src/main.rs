use reverse_string::*;

fn main() {
    println!("{}", rev_str("Hello, world!"));
    println!("{}", rev_str("Hello, my name is Roman"));
    println!("{}", rev_str("I have a nice car!"));
    println!("{}", rev_str("How old are You"));
    println!("{}", rev_str("ex: this is an example água"));
}

// Here we are passing a shared, IMMUTABLE reference to the `rev_str` function
// and it returns a new `String` with the reversed content.