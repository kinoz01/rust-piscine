use box_recursion::*;

fn main() {
    let mut list = WorkEnvironment::new();

    list.add_worker("Marie", "CEO");
    list.add_worker("Monica", "Manager");
    list.add_worker("Ana", "Normal Worker");
    list.add_worker("Alice", "Normal Worker");

    println!("{:#?}", list);

    println!("{:?}", list.last_worker());

    list.remove_worker();
    list.remove_worker();
    list.remove_worker();

    println!("{:?}", list);

    list.remove_worker();

    println!("{:?}", list);
}