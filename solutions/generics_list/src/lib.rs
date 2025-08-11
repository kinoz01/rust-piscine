#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let next = self.head.take().map(Box::new);
        self.head = Some(Node { value, next });
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|b| *b);
        }
    }

    pub fn len(&self) -> usize {
        let mut n = 0;
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            n += 1;
            cur = node.next.as_deref();
        }
        n
    }
}
 