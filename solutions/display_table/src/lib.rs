use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Self {
        Self { headers: Vec::new(), body: Vec::new() }
    }

    pub fn add_row(&mut self, row: &[String]) {
        assert_eq!(self.headers.len(), row.len());
        self.body.push(row.to_vec());
    }

    fn col_widths(&self) -> Vec<usize> {
        let mut w: Vec<usize> = self.headers
            .iter()
            .map(|h| h.len())
            .collect();
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                if i < w.len() {
                    w[i] = w[i].max(cell.len());
                }
            }
        }
        w
    }
}

// center with left bias (extra space to the right)
fn center(s: &str, w: usize) -> String {
    if s.len() >= w {
        return s.to_string();
    }
    let pad = w - s.len();
    let left = pad / 2;
    let right = pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

fn print_row(f: &mut fmt::Formatter, widths: &[usize], cells: &[String]) -> fmt::Result {
    write!(f, "|")?;
    for (i, w) in widths.iter().enumerate() {
        let cell = cells.get(i).map(String::as_str).unwrap_or("");
        write!(f, " {} |", center(cell, *w))?;
    }
    writeln!(f)
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.is_empty() {
            return Ok(());
        }

        let widths = self.col_widths();

        // headers
        print_row(f, &widths, &self.headers)?;

        // separator like: |----+-----+---|
        write!(f, "|")?;
        for (i, w) in widths.iter().enumerate() {
            write!(f, "{}", "-".repeat(w + 2))?;
            if i + 1 == widths.len() {
                write!(f, "|\n")?;
            } else {
                write!(f, "+")?;
            }
        }

        // body
        for row in &self.body {
            print_row(f, &widths, row)?;
        }

        Ok(())
    }
}
