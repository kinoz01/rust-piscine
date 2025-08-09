use lunch_queue::*;

// Example :
fn main() {
    let mut list = Queue::new();
    list.add(String::from("Marie"), 20);
    list.add(String::from("Monica"), 15);
    list.add(String::from("Ana"), 5);
    list.add(String::from("Alice"), 35);
    println!("{:?}", list);

    // output:
    // Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Marie", discount: 20, next_person: None }) }) }) }) }
    println!("{:?}", list.search("Marie"));
    println!("{:?}", list.search("Alice"));
    println!("{:?}", list.search("someone"));
    // output:
    // Some(("Marie", 20))
    // Some(("Alice", 35))
    // None

    println!("removed {:?}", list.rm());
    println!("removed {:?}", list.rm());
    println!("list {:?}", list);
    list.invert_queue();
    println!("inverted list {:?}", list);
    // output:
    // removed Some(("Marie", 20))
    // list Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: None }) }) }) }
}

