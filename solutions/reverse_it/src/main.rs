use reverse_it::*;

fn main() {
    println!("{}", reverse_it(123456789)); // "987654321123456789"
    println!("{}", reverse_it(-123));      // "-321123"
    println!("{}", reverse_it(123));       // Expected: "321123"
    println!("{}", reverse_it(123456789)); // Expected: "987654321123456789"
    println!("{}", reverse_it(0));         // Expected: "00"
    println!("{}", reverse_it(-123));      // Expected: "-321123"
    println!("{}", reverse_it(1));         // Expected: "11"
    println!("{}", reverse_it(i32::MIN));  // Expected: "-84638474122147483648"
    println!("{}", reverse_it(i32::MAX));  // Expected: "74638474122147483647"
    println!("{}", reverse_it(i32::MIN));  // Again:    "-84638474122147483648"
    println!("{}", reverse_it(i32::MAX));  // Again:    "74638474122147483647"
}
