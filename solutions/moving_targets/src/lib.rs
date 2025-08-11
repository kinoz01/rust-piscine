pub struct Field {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: Target,
    next: Link,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}
impl Field {
    pub fn new() -> Self {
        Self{head: None}
    }
    pub fn push(&mut self, target: Target) {
        self.head = Some(Box::new(
            Node {
                elem: target,
                next: self.head.take()
            }
        ))
    }
    pub fn pop(&mut self) -> Option<Target> {
        self.head.take().map(|node| {
            let Node {elem, next} = *node;
            self.head = next;
            elem
        })
    }
    pub fn peek(&self) -> Option<&Target> {
        self.head.as_ref().map(|n| &n.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut Target> {
        self.head.as_mut().map(|n| &mut n.elem)
    }
}