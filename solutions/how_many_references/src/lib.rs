use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Self { ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    // remove only entries that are the exact same Rc allocation
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|x| !Rc::ptr_eq(x, &element));
    }
}

pub fn how_many_references(r: &Rc<String>) -> usize {
    Rc::strong_count(r)
}
