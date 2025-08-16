// Create a struct called Person that has two fields: name of type
// string slice (&str) and age of type u8
// and create the associated function new which creates a new person
// with age 0 and with the name given

use lifetimes::Person;

fn main() {
    let person = Person::new("Leo");

    println!("Person = {:?}", person);
}

