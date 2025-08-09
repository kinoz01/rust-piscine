use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug)]
pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: Cell<usize>,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
            value: Cell::new(0),
            max,
        }
    }

    pub fn set_value<T>(&self, v: &Rc<T>) {
        let count = Rc::strong_count(v);
        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
            return;
        }
        self.value.set(count);
        let pct = count * 100 / self.max;
        if pct > 70 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                pct
            ));
        }
    }

    pub fn peek<T>(&self, v: &Rc<T>) {
        let count = Rc::strong_count(v);
        let pct = count * 100 / self.max;
        self.messages
            .borrow_mut()
            .push(format!("Info: This value would use {}% of your quota", pct));
    }
}
