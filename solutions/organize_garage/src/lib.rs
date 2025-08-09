use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Garage<T> {
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T> Garage<T> where T: Add<Output = T> + Copy {
    pub fn move_to_right(&mut self) {
        if let Some(l) = self.left.take() {
            self.right = Some(
                self.right
                    .map(|r| r + l)
                    .unwrap_or(l)
            );
        }
    }

    pub fn move_to_left(&mut self) {
        if let Some(r) = self.right.take() {
            self.left = Some(
                self.left
                    .map(|l| l + r)
                    .unwrap_or(r)
            );
        }
    }
}
