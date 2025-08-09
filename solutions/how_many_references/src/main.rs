use how_many_references::*;
use std::rc::Rc;

fn main() {
    let a = Rc::new("a".to_owned());
    let b = Rc::new("b".to_owned());
    let c = Rc::new("c".to_owned());

    let a1 = Rc::new("a".to_owned());

    let mut new_node = Node::new(vec![a.clone()]);
    new_node.add_element(b.clone());
    new_node.add_element(a.clone());
    new_node.add_element(c.clone());
    new_node.add_element(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("b: {:?}", how_many_references(&b));
    println!("c: {:?}", how_many_references(&c));
    new_node.rm_all_ref(a1.clone());
    new_node.rm_all_ref(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("b: {:?}", how_many_references(&b));
    println!("c: {:?}", how_many_references(&c));
}