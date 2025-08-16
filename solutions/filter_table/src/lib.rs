#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Self {
        Self { headers: vec![], body: vec![] }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }

    pub fn filter_col<F>(&self, filter: F) -> Option<Self> where F: Fn(&str) -> bool {
        let keep: Vec<_> = self.headers
            .iter()
            .enumerate()
            .filter_map(|(i, h)| filter(h).then_some(i))
            .collect();

        Some(Self {
            headers: keep
                .iter()
                .map(|&i| self.headers[i].clone())
                .collect(),
            body: self.body
                .iter()
                .map(|row|
                    keep
                        .iter()
                        .map(|&i| row[i].clone())
                        .collect()
                )
                .collect(),
        })
    }

    pub fn filter_row<F>(&self, col_name: &str, filter: F) -> Option<Self> where F: Fn(&str) -> bool {
        self.headers
            .iter()
            .position(|h| h == col_name)
            .map(|col| Self {
                headers: self.headers.clone(),
                body: self.body
                    .iter()
                    .filter(|row| filter(&row[col]))
                    .cloned()
                    .collect(),
            })
    }
}
