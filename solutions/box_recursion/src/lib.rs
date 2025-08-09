#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match () {
            _ if s.eq_ignore_ascii_case("ceo") => Role::CEO,
            _ if s.eq_ignore_ascii_case("manager") => Role::Manager,
            _ => Role::Worker,
        }
    }
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link, // head of the list (top of stack)
}

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link, // pointer to the next node
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let node = Box::new(Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        });
        self.grade = Some(node);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(node) = self.grade.take() {
            let Worker { name, next, .. } = *node;
            self.grade = next;
            Some(name)
        } else {
            None
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade.as_ref().map(|w| (w.name.clone(), w.role))
    }
}
