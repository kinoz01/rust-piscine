use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Garage<T> {
    pub left: Option<T>,
    pub right: Option<T>
}

impl<T> Garage<T> where T: Add<Output =T> {
    pub fn move_to_right(&mut self) {
        match (self.left.take(), self.right.take()) {
            (Some(l), Some(r)) => self.right = Some(l+r),
            (Some(l), None) => self.right = Some(l),
            (None, r) => self.right = r
        }
    }

    pub fn move_to_left(&mut self) {
        match (self.left.take(), self.right.take()) {
            (Some(l), Some(r)) => self.left = Some(l+r),
            (None, Some(r)) => self.left = Some(r),
            (l, None) => self.left = l
        }
    }
}
