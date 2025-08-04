#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        let mut ch = [self.r, self.g, self.b, self.a];

        let mut i = None;
        let mut j = None;
        for (idx, &val) in ch.iter().enumerate() {
            if val == first  && i.is_none() { i = Some(idx); }
            if val == second && j.is_none() { j = Some(idx); }
        }

        if let (Some(i), Some(j)) = (i, j) {
            if i != j {
                ch.swap(i, j);
            }
        }

        Color { r: ch[0], g: ch[1], b: ch[2], a: ch[3] }
    }
}
