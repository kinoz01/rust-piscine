pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    // push to head (latest joins at front)
    pub fn add(&mut self, name: String, discount: i32) {
        let new = Box::new(Person {
            name,
            discount,
            next_person: self.node.take(),
        });
        self.node = Some(new);
    }

    // reverse the linked list
    pub fn invert_queue(&mut self) {
        let mut prev: Link = None;
        let mut cur = self.node.take();
        while let Some(mut node) = cur {
            let next = node.next_person.take();
            node.next_person = prev;
            prev = Some(node);
            cur = next;
        }
        self.node = prev;
    }

    // remove FIFO (tail)
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut link = &mut self.node;
        loop {
            match link {
                None => return None,
                Some(n) if n.next_person.is_none() => {
                    let tail = link.take().unwrap();
                    let Person { name, discount, .. } = *tail;
                    return Some((name, discount));
                }
                Some(n) => link = &mut n.next_person,
            }
        }
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut cur = self.node.as_ref();
        while let Some(n) = cur {
            if n.name == name {
                return Some((n.name.clone(), n.discount));
            }
            cur = n.next_person.as_ref();
        }
        None
    }
}
